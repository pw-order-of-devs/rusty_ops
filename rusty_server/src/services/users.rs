use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::user::{PagedUsers, RegisterUser, User, UserModel};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;

use crate::services::{roles, shared};

const USERS_INDEX: &str = "users";

// query

pub async fn get_all(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<UserModel>, RustyError> {
    shared::get_all(db, USERS_INDEX, filter, options, false).await
}

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

pub async fn get_by_username(db: &DbClient, username: &str) -> Result<Option<User>, RustyError> {
    shared::get_one(
        db,
        USERS_INDEX,
        &json!({ "username": { "equals": username } }),
    )
    .await
}

// mutate

pub async fn create(db: &DbClient, user: RegisterUser) -> Result<String, RustyError> {
    if get_all(
        db,
        &Some(json!({ "username": { "equals": user.username } })),
        &None,
    )
    .await?
    .is_empty()
    {
        let user_id = shared::create(db, USERS_INDEX, user, |r| User::from(&r)).await?;
        match roles::assign(db, &user_id, None, Some("USERS")).await {
            Ok(_) => log::info!("added user {user_id} to group `USERS`"),
            Err(err) => log::warn!("error while adding user `{user_id}` to group `USERS`: {err}"),
        };
        Ok(user_id)
    } else {
        Err(RustyError::ValidationError(
            "user already exists".to_string(),
        ))
    }
}
