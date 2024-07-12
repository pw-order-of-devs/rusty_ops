use async_graphql::SelectionField;
use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::jobs::JobModel;
use domain::projects::{Project, ProjectModel, RegisterProject};
use persist::db_client::DbClient;

use crate::services::shared::get_username_claim;
use crate::services::{jobs, project_groups, shared};

const PROJECTS_INDEX: &str = "projects";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
    inner: &[SelectionField<'_>],
) -> Result<Vec<ProjectModel>, RustyError> {
    let entries = shared::get_all::<Project>(db, PROJECTS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if auth::authorize(db, &username, &format!("PROJECTS:READ:ID[{}]", entry.id))
            .await
            .is_ok()
        {
            filtered.push(ProjectModel::from(&entry));
        }
    }
    if inner.iter().map(|f| f.name()).any(|f| f == "jobs") {
        for f in &mut filtered {
            if let Ok(jobs) = get_jobs_for_project(db, cred, &f.id, inner).await {
                f.jobs = jobs;
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
) -> Result<Option<ProjectModel>, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, &format!("PROJECTS:READ:ID[{id}]")).await?;
    if let Some(project) = shared::get_by_id::<Project>(db, PROJECTS_INDEX, id).await? {
        let mut model = ProjectModel::from(&project);
        if inner.iter().map(|f| f.name()).any(|f| f == "jobs") {
            if let Ok(jobs) = get_jobs_for_project(db, cred, &project.id, inner).await {
                model.jobs = jobs;
            }
        }
        Ok(Some(model))
    } else {
        Ok(None)
    }
}

async fn get_jobs_for_project(
    db: &DbClient,
    cred: &Credential,
    id: &str,
    inner: &[SelectionField<'_>],
) -> Result<Vec<JobModel>, RustyError> {
    let jobs_inner = if let Some(field) = inner.iter().find(|f| f.name() == "jobs") {
        field.selection_set().collect()
    } else {
        vec![]
    };

    jobs::get_all(
        db,
        cred,
        &Some(json!({ "project_id": { "equals": id } })),
        &None,
        &jobs_inner,
    )
    .await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    project: RegisterProject,
) -> Result<String, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, "PROJECTS:CREATE").await?;
    let group_id = project.clone().group_id.unwrap_or_default();
    if !group_id.is_empty()
        && project_groups::get_by_id(db, cred, &group_id, &[])
            .await?
            .is_none()
    {
        Err(RustyError::ValidationError(
            "project group not found".to_string(),
        ))
    } else {
        shared::create(db, PROJECTS_INDEX, project, |r| Project::from(&r)).await
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    shared::check_project_write_permission(db, cred, id).await?;
    shared::delete_by_id::<Project>(db, PROJECTS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, PROJECTS_INDEX).await
}
