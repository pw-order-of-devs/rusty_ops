use async_graphql::futures_util::Stream;
use serde_json::{json, Value};

use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::jobs::Job;
use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;

use crate::services::shared::get_username_claim;
use crate::services::{agents, jobs, shared};

const PIPELINES_INDEX: &str = "pipelines";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Pipeline>, RustyError> {
    let entries = shared::get_all::<Pipeline>(db, PIPELINES_INDEX, filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if let Some(job) = shared::get_by_id::<Job>(db, "jobs", &entry.job_id).await? {
            if auth::authorize(
                db,
                &username,
                &format!("PROJECTS:READ:ID[{}]", job.project_id),
            )
            .await
            .is_ok()
            {
                filtered.push(entry);
            }
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Option<Pipeline>, RustyError> {
    if let Some(pipeline) = shared::get_by_id::<Pipeline>(db, PIPELINES_INDEX, id).await? {
        if let Some(job) = shared::get_by_id::<Job>(db, "jobs", id).await? {
            let username = get_username_claim(cred)?;
            auth::authorize(
                db,
                &username,
                &format!("PROJECTS:READ:ID[{}]", job.project_id),
            )
            .await?;
            Ok(Some(pipeline))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    pipeline: RegisterPipeline,
) -> Result<String, RustyError> {
    if let Some(job) = jobs::get_by_id(db, cred, &pipeline.job_id, &None, &[]).await? {
        shared::check_project_write_permission(db, cred, &job.project_id).await?;
        let pipelines_count = get_all(
            db,
            cred,
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
    } else {
        Err(RustyError::ValidationError("job not found".to_string()))
    }
}

pub async fn assign(
    db: &DbClient,
    cred: &Credential,
    pipeline_id: &str,
    agent_id: &str,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, cred, pipeline_id).await? {
        if let Some(job) = jobs::get_by_id(db, cred, &pipe.job_id, &None, &[]).await? {
            shared::check_project_write_permission(db, cred, &job.project_id).await?;
            if pipe.status == PipelineStatus::Defined && pipe.agent_id.is_none() {
                pipe.status = PipelineStatus::Assigned;
                pipe.agent_id = Some(agent_id.to_string());

                let limit = var_or_default("AGENT_MAX_ASSIGNED_JOBS", 1);
                let condition = json!({ "status": { "equals": "ASSIGNED" }, "agent_id": { "equals": agent_id } });
                if get_all(db, cred, &Some(condition), &None).await?.len() < limit {
                    db.update(PIPELINES_INDEX, pipeline_id, &pipe).await
                } else {
                    let message = format!(
                        "`pipelines::assign` - exceeded {limit} pipeline(s) assigned to agent"
                    );
                    log::debug!("{message}");
                    Err(RustyError::AsyncGraphqlError(message))
                }
            } else {
                let message = "`pipelines::assign` - pipeline already assigned".to_string();
                log::debug!("{message}");
                Err(RustyError::AsyncGraphqlError(message))
            }
        } else {
            Err(RustyError::UnauthorizedError)
        }
    } else {
        let message = "`pipelines::assign` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn reset(
    db: &DbClient,
    cred: &Credential,
    pipeline_id: &str,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, cred, pipeline_id).await? {
        if let Some(job) = jobs::get_by_id(db, cred, &pipe.job_id, &None, &[]).await? {
            shared::check_project_write_permission(db, cred, &job.project_id).await?;
            if [PipelineStatus::Assigned, PipelineStatus::InProgress].contains(&pipe.status)
                && agents::get_by_id(db, cred, &pipe.agent_id.unwrap())
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
            Err(RustyError::UnauthorizedError)
        }
    } else {
        let message = "`pipelines::reset` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn set_running(
    db: &DbClient,
    cred: &Credential,
    pipeline_id: &str,
    agent_id: &str,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, cred, pipeline_id).await? {
        if let Some(job) = jobs::get_by_id(db, cred, &pipe.job_id, &None, &[]).await? {
            shared::check_project_write_permission(db, cred, &job.project_id).await?;
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
            Err(RustyError::UnauthorizedError)
        }
    } else {
        let message = "`pipelines::setRunning` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn finalize(
    db: &DbClient,
    cred: &Credential,
    pipeline_id: &str,
    agent_id: &str,
    status: PipelineStatus,
) -> Result<String, RustyError> {
    if let Some(mut pipe) = get_by_id(db, cred, pipeline_id).await? {
        if let Some(job) = jobs::get_by_id(db, cred, &pipe.job_id, &None, &[]).await? {
            shared::check_project_write_permission(db, cred, &job.project_id).await?;
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
            Err(RustyError::UnauthorizedError)
        }
    } else {
        let message = "`pipelines::finalize` - pipeline not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    if let Some(pipe) = get_by_id(db, cred, id).await? {
        if let Some(job) = jobs::get_by_id(db, cred, &pipe.job_id, &None, &[]).await? {
            shared::check_project_write_permission(db, cred, &job.project_id).await?;
            shared::delete_by_id::<Pipeline>(db, PIPELINES_INDEX, id).await
        } else {
            Ok(0)
        }
    } else {
        Ok(0)
    }
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, PIPELINES_INDEX).await
}

// subscriptions
pub fn inserted_stream(db: &DbClient) -> impl Stream<Item = Option<Pipeline>> + '_ {
    db.change_stream(PIPELINES_INDEX)
}
