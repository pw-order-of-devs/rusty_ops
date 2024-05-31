use async_graphql::futures_util::Stream;
use serde_json::{json, Value};

use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::pipelines::{PagedPipelines, Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;

use crate::services::{agents, jobs, shared};

const PIPELINES_INDEX: &str = "pipelines";

// query

pub async fn get_all(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Pipeline>, RustyError> {
    shared::get_all(db, PIPELINES_INDEX, filter, options, false).await
}

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedPipelines, RustyError> {
    let count = shared::get_total_count::<Pipeline>(db, PIPELINES_INDEX, filter).await?;
    let entries = shared::get_all(db, PIPELINES_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedPipelines {
        total: count,
        page,
        page_size,
        entries,
    })
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Pipeline>, RustyError> {
    shared::get_by_id(db, PIPELINES_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, pipeline: RegisterPipeline) -> Result<String, RustyError> {
    if jobs::get_by_id(db, &pipeline.job_id).await?.is_none() {
        Err(RustyError::ValidationError("job not found".to_string()))
    } else {
        let pipelines_count = get_all(
            db,
            &Some(json!({ "job_id": { "equals": pipeline.job_id } })),
            &None,
        )
        .await?
        .len() as u64;

        let register = pipeline.clone();
        let mut pipeline = Pipeline::from(&pipeline);
        pipeline.number = pipelines_count + 1;
        pipeline.register_date = chrono::Utc::now().to_rfc3339();
        shared::create(db, PIPELINES_INDEX, register, |_| pipeline).await
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
            let condition =
                json!({ "status": { "equals": "ASSIGNED" }, "agent_id": { "equals": agent_id } });
            if get_all(db, &Some(condition), &None).await?.len() < limit {
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

pub async fn reset(db: &DbClient, pipeline_id: &str) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, pipeline_id).await? {
        if [PipelineStatus::Assigned, PipelineStatus::InProgress].contains(&pipe.status)
            && agents::get_by_id(db, &pipe.agent_id.unwrap())
                .await?
                .is_none()
        {
            pipe.status = PipelineStatus::Defined;
            pipe.agent_id = None;
            db.update(PIPELINES_INDEX, pipeline_id, &pipe).await
        } else {
            let message = "`pipelines::reset` - cannot update".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError(message))
        }
    } else {
        let message = "`pipelines::reset` - pipeline not found".to_string();
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
    shared::delete_by_id::<Pipeline>(db, PIPELINES_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, PIPELINES_INDEX).await
}

// subscriptions
pub fn inserted_stream(db: &DbClient) -> impl Stream<Item = Option<Pipeline>> + '_ {
    db.change_stream(PIPELINES_INDEX)
}
