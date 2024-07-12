use async_graphql::SelectionField;
use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::jobs::{Job, JobModel, RegisterJob};
use domain::pipelines::Pipeline;
use persist::db_client::DbClient;

use crate::services::shared::get_username_claim;
use crate::services::{pipelines, projects, shared};

const JOBS_INDEX: &str = "jobs";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
    inner: &[SelectionField<'_>],
) -> Result<Vec<JobModel>, RustyError> {
    let entries = shared::get_all::<Job>(db, JOBS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if auth::authorize(
            db,
            &username,
            &format!("PROJECTS:READ:ID[{}]", entry.project_id),
        )
        .await
        .is_ok()
        {
            filtered.push(JobModel::from(&entry));
        }
    }

    if inner.iter().map(|f| f.name()).any(|f| f == "pipelines") {
        for f in &mut filtered {
            if let Ok(pipelines) = get_pipelines_for_job(db, cred, &f.id).await {
                f.pipelines = pipelines;
            }
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
    inner: &[SelectionField<'_>],
) -> Result<Option<JobModel>, RustyError> {
    let job = shared::get_by_id::<Job>(db, JOBS_INDEX, id).await?;
    if let Some(job) = job {
        let username = get_username_claim(cred)?;
        auth::authorize(
            db,
            &username,
            &format!("PROJECTS:READ:ID[{}]", job.project_id),
        )
        .await?;
        projects::get_by_id(db, cred, &job.project_id, &[]).await?;
        let mut model = JobModel::from(&job);
        if inner.iter().map(|f| f.name()).any(|f| f == "pipelines") {
            if let Ok(pipelines) = get_pipelines_for_job(db, cred, &job.id).await {
                model.pipelines = pipelines;
            }
        }
        Ok(Some(model))
    } else {
        Ok(None)
    }
}

async fn get_pipelines_for_job(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Vec<Pipeline>, RustyError> {
    pipelines::get_all(
        db,
        cred,
        &Some(json!({ "job_id": { "equals": id } })),
        &None,
    )
    .await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    job: RegisterJob,
) -> Result<String, RustyError> {
    if let Some(project) = projects::get_by_id(db, cred, &job.project_id, &[]).await? {
        shared::check_project_write_permission(db, cred, &project.id).await?;
        shared::create(db, JOBS_INDEX, job, |r| Job::from(&r)).await
    } else {
        Err(RustyError::ValidationError("project not found".to_string()))
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    if let Some(job) = get_by_id(db, cred, id, &[]).await? {
        shared::check_project_write_permission(db, cred, &job.project_id).await?;
    }
    shared::delete_by_id::<Job>(db, JOBS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, JOBS_INDEX).await
}
