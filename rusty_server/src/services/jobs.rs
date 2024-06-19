use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::jobs::{Job, RegisterJob};
use persist::db_client::DbClient;

use crate::services::{projects, shared};

const JOBS_INDEX: &str = "jobs";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Job>, RustyError> {
    let entries = shared::get_all::<Job>(db, JOBS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    for entry in entries {
        if projects::get_by_id(db, cred, &entry.project_id)
            .await?
            .is_some()
        {
            filtered.push(entry);
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Option<Job>, RustyError> {
    let job = shared::get_by_id::<Job>(db, JOBS_INDEX, id).await?;
    if let Some(job) = job {
        shared::has_permission(db, cred, &job.project_id, ("PROJECTS", "READ")).await?;
        Ok(Some(job))
    } else {
        Ok(None)
    }
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    job: RegisterJob,
) -> Result<String, RustyError> {
    shared::has_permission(db, cred, &job.project_id, ("PROJECTS", "WRITE")).await?;
    if projects::get_by_id(db, cred, &job.project_id)
        .await?
        .is_none()
    {
        Err(RustyError::ValidationError("project not found".to_string()))
    } else {
        shared::create(db, JOBS_INDEX, job, |r| Job::from(&r)).await
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    let _ = get_by_id(db, cred, id).await?;
    shared::delete_by_id::<Job>(db, JOBS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, JOBS_INDEX).await
}
