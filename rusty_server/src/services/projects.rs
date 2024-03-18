use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::projects::{Project, RegisterProject};
use persist::db_client::DbClient;

const PROJECTS_INDEX: &str = "projects";

// query

pub async fn get_all(
    db: &DbClient,
    filter: Option<Value>,
    options: Option<SearchOptions>,
) -> Result<Vec<Project>, RustyError> {
    let entries = db
        .get_all(PROJECTS_INDEX, filter, options)
        .await
        .map_err(|err| {
            log::error!("`projects::get`: {err}");
            err
        })?;
    Ok(entries)
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Project>, RustyError> {
    let entry = db
        .get_one::<Project>(PROJECTS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`projects::getById`: {err}");
            err
        })?;
    Ok(entry)
}

// mutate

pub async fn create(db: &DbClient, project: RegisterProject) -> Result<String, RustyError> {
    project.validate().map_err(|err| {
        log::error!("`projects::create`: {err}");
        err
    })?;

    let id = db
        .create(PROJECTS_INDEX, &Project::from(&project))
        .await
        .map_err(|err| {
            log::error!("`projects::create`: {err}");
            err
        })?;
    Ok(id)
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    let id = db
        .delete_one::<Project>(PROJECTS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`projects::deleteById`: {err}");
            err
        })?;
    Ok(id)
}
