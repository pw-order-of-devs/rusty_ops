use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::{RegisterUser, User, UserModel};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;

use crate::services::shared::get_username_claim;
use crate::services::{roles, shared};

const USERS_INDEX: &str = "users";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<UserModel>, RustyError> {
    auth::authorize(db, &get_username_claim(cred)?, "USERS:READ").await?;
    shared::get_all(db, USERS_INDEX, filter, options).await
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Option<UserModel>, RustyError> {
    auth::authorize(db, &get_username_claim(cred)?, "USERS:READ").await?;
    shared::get_by_id(db, USERS_INDEX, id).await
}

pub async fn get_by_username(
    db: &DbClient,
    cred: &Credential,
    username: &str,
) -> Result<Option<User>, RustyError> {
    auth::authorize(db, &get_username_claim(cred)?, "USERS:READ").await?;
    shared::get_one(
        db,
        USERS_INDEX,
        &json!({ "username": { "equals": username } }),
    )
    .await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    user: RegisterUser,
) -> Result<String, RustyError> {
    let users_by_username = get_all(
        db,
        cred,
        &Some(json!({ "username": { "equals": user.username } })),
        &None,
    )
    .await?;
    let users_by_email = get_all(
        db,
        cred,
        &Some(json!({ "email": { "equals": user.email } })),
        &None,
    )
    .await?;

    if !users_by_username.is_empty() {
        Err(RustyError::ValidationError(
            "user already exists - username taken".to_string(),
        ))
    } else if !users_by_email.is_empty() {
        Err(RustyError::ValidationError(
            "user already exists - email address taken".to_string(),
        ))
    } else {
        let user_id = shared::create(db, USERS_INDEX, user, |r| User::from(&r)).await?;
        match roles::assign(db, cred, &user_id, None, Some("USERS")).await {
            Ok(_) => log::info!("added user {user_id} to group `USERS`"),
            Err(err) => log::warn!("error while adding user `{user_id}` to group `USERS`: {err}"),
        };
        Ok(user_id)
    }
}
