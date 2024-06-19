use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::projects::{Project, RegisterProject};
use persist::db_client::DbClient;

use crate::services::{project_groups, shared};

const PROJECTS_INDEX: &str = "projects";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Project>, RustyError> {
    let entries = shared::get_all::<Project>(db, PROJECTS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    for entry in entries {
        let has = shared::has_permission(db, cred, &entry.id, ("PROJECTS", "READ")).await;
        if has.is_ok() {
            filtered.push(entry);
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Option<Project>, RustyError> {
    shared::has_permission(db, cred, id, ("PROJECTS", "READ")).await?;
    shared::get_by_id(db, PROJECTS_INDEX, id).await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    project: RegisterProject,
) -> Result<String, RustyError> {
    shared::has_permission(db, cred, "", ("PROJECTS", "CREATE")).await?;
    let group_id = project.clone().group_id.unwrap_or_default();
    if !group_id.is_empty() && project_groups::get_by_id(db, &group_id).await?.is_none() {
        Err(RustyError::ValidationError(
            "project group not found".to_string(),
        ))
    } else {
        shared::create(db, PROJECTS_INDEX, project, |r| Project::from(&r)).await
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    let _ = get_by_id(db, cred, id).await?;
    shared::delete_by_id::<Project>(db, PROJECTS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, PROJECTS_INDEX).await
}
