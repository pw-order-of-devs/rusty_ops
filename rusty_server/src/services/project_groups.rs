use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::projects::{Group, PagedGroups, RegisterGroup};
use persist::db_client::DbClient;

use crate::services::shared;

const GROUPS_INDEX: &str = "project_groups";

// query

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedGroups, RustyError> {
    let count = shared::get_total_count::<Group>(db, GROUPS_INDEX, filter).await?;
    let entries = shared::get_all(db, GROUPS_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedGroups {
        total: count,
        page,
        page_size,
        entries,
    })
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
