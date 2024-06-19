use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::projects::{Group, RegisterGroup};
use persist::db_client::DbClient;

use crate::services::shared;

const GROUPS_INDEX: &str = "project_groups";

// query

pub async fn get_all(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Group>, RustyError> {
    shared::get_all(db, GROUPS_INDEX, filter, options).await
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Group>, RustyError> {
    shared::get_by_id(db, GROUPS_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, group: RegisterGroup) -> Result<String, RustyError> {
    shared::create(db, GROUPS_INDEX, group, |r| Group::from(&r)).await
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    shared::delete_by_id::<Group>(db, GROUPS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, GROUPS_INDEX).await
}
