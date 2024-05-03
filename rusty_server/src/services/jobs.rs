use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::jobs::{Job, PagedJobs, RegisterJob};
use persist::db_client::DbClient;

use crate::services::{projects, shared};

const JOBS_INDEX: &str = "jobs";

// query

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedJobs, RustyError> {
    let count = shared::get_total_count::<Job>(db, JOBS_INDEX, filter).await?;
    let entries = shared::get_all(db, JOBS_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedJobs {
        total: count,
        page,
        page_size,
        entries,
    })
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Job>, RustyError> {
    shared::get_by_id(db, JOBS_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, job: RegisterJob) -> Result<String, RustyError> {
    if projects::get_by_id(db, &job.project_id).await?.is_none() {
        Err(RustyError::ValidationError("project not found".to_string()))
    } else {
        shared::create(db, JOBS_INDEX, job, |r| Job::from(&r)).await
    }
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    shared::delete_by_id::<Job>(db, JOBS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, JOBS_INDEX).await
}
