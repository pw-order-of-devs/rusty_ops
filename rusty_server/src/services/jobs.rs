use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::jobs::{Job, RegisterJob};
use persist::db_client::DbClient;

use crate::services::projects;

const JOBS_INDEX: &str = "jobs";

// query

pub async fn get_all(
    db: &DbClient,
    filter: Option<Value>,
    options: Option<SearchOptions>,
) -> Result<Vec<Job>, RustyError> {
    let entries = db
        .get_all(JOBS_INDEX, filter, options)
        .await
        .map_err(|err| {
            log::error!("`jobs::get`: {err}");
            err
        })?;
    Ok(entries)
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Job>, RustyError> {
    let entry = db
        .get_one::<Job>(JOBS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`jobs::getById`: {err}");
            err
        })?;
    Ok(entry)
}

// mutate

pub async fn create(db: &DbClient, job: RegisterJob) -> Result<String, RustyError> {
    job.validate().map_err(|err| {
        log::error!("`jobs::create`: {err}");
        err
    })?;

    if projects::get_by_id(db, &job.project_id).await?.is_none() {
        Err(RustyError::ValidationError(
            json!({
                "errors": [],
                "properties": {"project_id": {"errors": ["project not found"]}}
            })
            .to_string(),
        ))
    } else {
        let id = db
            .create(JOBS_INDEX, &Job::from(&job))
            .await
            .map_err(|err| {
                log::error!("`jobs::create`: {err}");
                err
            })?;
        Ok(id)
    }
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    let id = db
        .delete_one::<Job>(JOBS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`jobs::deleteById`: {err}");
            err
        })?;
    Ok(id)
}