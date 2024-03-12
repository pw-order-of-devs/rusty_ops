use async_graphql::{Context, Object};
use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::jobs::{Job, RegisterJob};
use domain::projects::Project;
use persist::Persistence;

use crate::gql::get_db_client;
use crate::gql::projects::PROJECTS_INDEX;

pub(crate) const JOBS_INDEX: &str = "jobs";

pub struct JobsQuery;

#[Object]
impl JobsQuery {
    // jobs interface
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Job>, RustyError> {
        log::debug!("handling `get_jobs` request");
        let entries = get_db_client(ctx)?
            .get_all(JOBS_INDEX, filter, options)
            .await?;
        log::debug!("`get_jobs`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Job>, RustyError> {
        log::debug!("handling `get_job_by_id` request");
        get_db_client(ctx)?.get_by_id(JOBS_INDEX, &id).await
    }
}

pub struct JobsMutation;

#[Object]
impl JobsMutation {
    // jobs interface
    async fn register(
        &self,
        ctx: &Context<'_>,
        job: RegisterJob,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `register_job` request");
        job.validate()?;
        let db = get_db_client(ctx)?;
        if db
            .get_by_id::<Project>(PROJECTS_INDEX, &job.project_id)
            .await?
            .is_none()
        {
            Err(RustyError::ValidationError {
                message: json!({
                    "errors": [],
                    "properties": {"project_id": {"errors": ["project not found"]}}
                })
                .to_string(),
            })
        } else {
            db.create(JOBS_INDEX, &Job::from(&job)).await
        }
    }

    async fn delete_one(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `delete_job` request");
        get_db_client(ctx)?.delete(JOBS_INDEX, &id).await
    }
}
