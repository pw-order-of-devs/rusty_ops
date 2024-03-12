use async_graphql::{Context, Object};
use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::jobs::Job;
use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use persist::Persistence;

use crate::gql::get_db_client;
use crate::gql::jobs::JOBS_INDEX;

pub(crate) const PIPELINES_INDEX: &str = "pipelines";

pub struct PipelinesQuery;

#[Object]
impl PipelinesQuery {
    // pipelines interface
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Pipeline>, RustyError> {
        log::debug!("handling `get_pipelines` request");
        let entries = get_db_client(ctx)?
            .get_all(PIPELINES_INDEX, filter, options)
            .await?;
        log::debug!("`get_pipelines`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Pipeline>, RustyError> {
        log::debug!("handling `get_pipeline_by_id` request");
        get_db_client(ctx)?.get_by_id(PIPELINES_INDEX, &id).await
    }
}

pub struct PipelinesMutation;

#[Object]
impl PipelinesMutation {
    // pipelines interface
    async fn register(
        &self,
        ctx: &Context<'_>,
        pipeline: RegisterPipeline,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `register_pipeline` request");
        let db = get_db_client(ctx)?;
        if db
            .get_by_id::<Job>(JOBS_INDEX, &pipeline.job_id)
            .await?
            .is_none()
        {
            Err(RustyError::ValidationError {
                message: json!({
                    "errors": [],
                    "properties": {"job_id": {"errors": ["job not found"]}}
                })
                .to_string(),
            })
        } else {
            let pipelines_count = db
                .get_all::<Pipeline>(
                    PIPELINES_INDEX,
                    Some(json!({ "job_id": pipeline.job_id })),
                    None,
                )
                .await?
                .len() as u64;
            let mut pipeline = Pipeline::from(&pipeline);
            pipeline.number = pipelines_count + 1;
            pipeline.start_date = chrono::Utc::now().to_rfc3339();
            get_db_client(ctx)?.create(PIPELINES_INDEX, &pipeline).await
        }
    }

    // pipelines interface
    async fn assign(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `assign_pipeline` request");
        let db = get_db_client(ctx)?;
        let pipeline = db
            .get_by_id::<Pipeline>(PIPELINES_INDEX, &pipeline_id)
            .await?;

        if let Some(mut pipe) = pipeline {
            if pipe.status == PipelineStatus::Defined && pipe.agent_id.is_none() {
                pipe.status = PipelineStatus::Assigned;
                pipe.agent_id = Some(agent_id.to_string());
                db.update(PIPELINES_INDEX, &pipeline_id, &pipe).await
            } else {
                let message = "`assign_pipeline` - pipeline already assigned".to_string();
                log::debug!("{message}");
                Err(RustyError::AsyncGraphqlError { message })
            }
        } else {
            let message = "`assign_pipeline` - pipeline not found".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError { message })
        }
    }

    async fn delete_one(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `delete_pipeline` request");
        get_db_client(ctx)?.delete(PIPELINES_INDEX, &id).await
    }
}
