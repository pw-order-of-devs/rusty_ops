use commons::errors::RustyError;
use domain::auth::user::{PagedUsers, RegisterUser, User, UserModel};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;
use serde_json::Value;

use crate::services::shared;

const USERS_INDEX: &str = "users";

// query

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedUsers, RustyError> {
    let count = shared::get_total_count::<UserModel>(db, USERS_INDEX, filter).await?;
    let entries = shared::get_all(db, USERS_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedUsers {
        total: count,
        page,
        page_size,
        entries,
    })
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<UserModel>, RustyError> {
    shared::get_by_id(db, USERS_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, user: RegisterUser) -> Result<String, RustyError> {
    shared::create(db, USERS_INDEX, user, |r| User::from(&r)).await
}
