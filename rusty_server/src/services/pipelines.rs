use async_graphql::futures_util::Stream;
use serde_json::{json, Value};
use serde_valid::Validate;

use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;

use crate::services::jobs;

const PIPELINES_INDEX: &str = "pipelines";

// query

pub async fn get_all(
    db: &DbClient,
    filter: Option<Value>,
    options: Option<SearchOptions>,
) -> Result<Vec<Pipeline>, RustyError> {
    let entries = db
        .get_all(PIPELINES_INDEX, filter, options)
        .await
        .map_err(|err| {
            log::error!("`pipelines::get`: {err}");
            err
        })?;
    Ok(entries)
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Pipeline>, RustyError> {
    let entry = db
        .get_one::<Pipeline>(PIPELINES_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`pipelines::getById`: {err}");
            err
        })?;
    Ok(entry)
}

// mutate

pub async fn create(db: &DbClient, pipeline: RegisterPipeline) -> Result<String, RustyError> {
    pipeline.validate().map_err(|err| {
        log::error!("`pipeline::create`: {err}");
        err
    })?;

    if jobs::get_by_id(db, &pipeline.job_id).await?.is_none() {
        Err(RustyError::ValidationError(
            json!({
                "errors": [],
                "properties": {"job_id": {"errors": ["job not found"]}}
            })
            .to_string(),
        ))
    } else {
        let pipelines_count = get_all(db, Some(json!({ "job_id": pipeline.job_id })), None)
            .await?
            .len() as u64;

        let mut pipeline = Pipeline::from(&pipeline);
        pipeline.number = pipelines_count + 1;
        pipeline.register_date = chrono::Utc::now().to_rfc3339();
        let id = db.create(PIPELINES_INDEX, &pipeline).await.map_err(|err| {
            log::error!("`pipelines::create`: {err}");
            err
        })?;
        Ok(id)
    }
}

pub async fn assign(
    db: &DbClient,
    pipeline_id: &str,
    agent_id: &str,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, pipeline_id).await? {
        if pipe.status == PipelineStatus::Defined && pipe.agent_id.is_none() {
            pipe.status = PipelineStatus::Assigned;
            pipe.agent_id = Some(agent_id.to_string());

            let limit = var_or_default("AGENT_MAX_ASSIGNED_JOBS", 1);
            let condition = json!({ "status": "ASSIGNED", "agent_id": agent_id });
            if get_all(db, Some(condition), None).await?.len() < limit {
                db.update(PIPELINES_INDEX, pipeline_id, &pipe).await
            } else {
                let message =
                    format!("`pipelines::assign` - exceeded {limit} pipeline(s) assigned to agent");
                log::debug!("{message}");
                Err(RustyError::AsyncGraphqlError(message))
            }
        } else {
            let message = "`pipelines::assign` - pipeline already assigned".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError(message))
        }
    } else {
        let message = "`pipelines::assign` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn set_running(
    db: &DbClient,
    pipeline_id: &str,
    agent_id: &str,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, pipeline_id).await? {
        if pipe.clone().agent_id.unwrap_or_else(String::new) == agent_id
            && pipe.clone().status == PipelineStatus::Assigned
        {
            pipe.status = PipelineStatus::InProgress;
            pipe.start_date = Some(chrono::Utc::now().to_rfc3339());
            db.update(PIPELINES_INDEX, pipeline_id, &pipe).await
        } else {
            let message = "`pipelines::setRunning` - cannot update".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError(message))
        }
    } else {
        let message = "`pipelines::setRunning` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn finalize(
    db: &DbClient,
    pipeline_id: &str,
    agent_id: &str,
    status: PipelineStatus,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, pipeline_id).await? {
        if pipe.clone().agent_id.unwrap_or_else(String::new) == agent_id
            && pipe.clone().status == PipelineStatus::InProgress
        {
            pipe.status = status;
            pipe.end_date = Some(chrono::Utc::now().to_rfc3339());
            db.update(PIPELINES_INDEX, pipeline_id, &pipe).await
        } else {
            let message = "`pipelines::finalize` - cannot update".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError(message))
        }
    } else {
        let message = "`pipelines::finalize` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    let id = db
        .delete_one::<Pipeline>(PIPELINES_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`pipelines::deleteById`: {err}");
            err
        })?;
    Ok(id)
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    let id = db.delete_all(PIPELINES_INDEX).await.map_err(|err| {
        log::error!("`pipelines::deleteAll`: {err}");
        err
    })?;
    Ok(id)
}

// subscriptions
pub fn inserted_stream(db: &DbClient) -> impl Stream<Item = Pipeline> + '_ {
    db.change_stream(PIPELINES_INDEX)
}
