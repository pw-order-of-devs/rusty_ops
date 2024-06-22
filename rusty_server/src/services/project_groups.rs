use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::projects::{Group, RegisterGroup};
use persist::db_client::DbClient;

use crate::services::shared;
use crate::services::shared::get_username_claim;

const GROUPS_INDEX: &str = "project_groups";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Group>, RustyError> {
    let entries = shared::get_all::<Group>(db, GROUPS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if auth::authorize(
            db,
            &username,
            &format!("PROJECT_GROUPS:READ:ID[{}]", entry.id),
        )
        .await
        .is_ok()
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
) -> Result<Option<Group>, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, &format!("PROJECT_GROUPS:READ:ID[{id}]")).await?;
    shared::get_by_id(db, GROUPS_INDEX, id).await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    group: RegisterGroup,
) -> Result<String, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, "PROJECT_GROUPS:CREATE").await?;
    shared::create(db, GROUPS_INDEX, group, |r| Group::from(&r)).await
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, &format!("PROJECT_GROUPS:WRITE:ID[{id}]")).await?;
    shared::delete_by_id::<Group>(db, GROUPS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, GROUPS_INDEX).await
}
