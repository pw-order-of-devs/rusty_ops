use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::projects::{PagedProjects, Project, RegisterProject};
use persist::db_client::DbClient;

use crate::services::shared;

const PROJECTS_INDEX: &str = "projects";

// query

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedProjects, RustyError> {
    let count = shared::get_total_count::<Project>(db, PROJECTS_INDEX, filter).await?;
    let entries = shared::get_all(db, PROJECTS_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedProjects {
        total: count,
        page,
        page_size,
        entries,
    })
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Project>, RustyError> {
    shared::get_by_id(db, PROJECTS_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, project: RegisterProject) -> Result<String, RustyError> {
    shared::create(db, PROJECTS_INDEX, project, |r| Project::from(&r)).await
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    shared::delete_by_id::<Project>(db, PROJECTS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, PROJECTS_INDEX).await
}
