use serde_json::{json, Value};

use crate::services::shared::get_username_claim;
use crate::services::{roles, shared};
use commons::errors::RustyError;
use commons::hashing::bcrypt;
use domain::auth::credentials::Credential;
use domain::auth::user::{
    RegisterUser, RegisterUserCredential, User, UserCredential, UserCredentialModel, UserModel,
};
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;
use persist::db_client::DbClient;
use secret::sc_client::ScClient;

const USERS_INDEX: &str = "users";
const CREDENTIALS_INDEX: &str = "credentials";
const PERMISSIONS_INDEX: &str = "permissions";

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

pub async fn get_current(
    db: &DbClient,
    cred: &Credential,
) -> Result<Option<UserModel>, RustyError> {
    if let Some(user) = get_by_username(db, &get_username_claim(cred)?).await? {
        Ok(Some(UserModel::from(&user)))
    } else {
        Ok(None)
    }
}

pub async fn get_by_username(db: &DbClient, username: &str) -> Result<Option<User>, RustyError> {
    shared::get_one(
        db,
        USERS_INDEX,
        &json!({ "username": { "equals": username } }),
    )
    .await
}

pub async fn get_credentials(
    db: &DbClient,
    sc: &ScClient,
    cred: &Credential,
    options: &Option<SearchOptions>,
    username: &str,
) -> Result<Vec<UserCredential>, RustyError> {
    assert_same_user(cred, username)?;

    if let Some(user) = get_by_username(db, username).await? {
        let mut credentials = vec![];
        for item in shared::get_all::<UserCredentialModel>(
            db,
            CREDENTIALS_INDEX,
            &Some(json!({ "user_id": { "equals": user.id } })),
            options,
        )
        .await?
        {
            let credential = UserCredential {
                id: item.id.clone(),
                name: item.name,
                token: sc.get(&item.id).await?.unwrap_or_default(),
                user_id: item.user_id,
            };
            credentials.push(credential);
        }
        Ok(credentials)
    } else {
        Err(RustyError::AsyncGraphqlError("user not found".to_string()))
    }
}

// mutate

pub async fn create(db: &DbClient, user: RegisterUser) -> Result<String, RustyError> {
    let users_by_username = get_all(
        db,
        &Credential::System,
        &Some(json!({ "username": { "equals": user.username } })),
        &None,
    )
    .await?;
    let users_by_email = get_all(
        db,
        &Credential::System,
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
        let user_id = shared::create_parse(db, USERS_INDEX, user, |r| User::from(&r)).await?;
        match roles::assign(db, &Credential::System, &user_id, None, Some("USERS")).await {
            Ok(_) => log::info!("added user {user_id} to group `USERS`"),
            Err(err) => log::warn!("error while adding user `{user_id}` to group `USERS`: {err}"),
        };
        Ok(user_id)
    }
}

pub async fn change_password(
    db: &DbClient,
    cred: &Credential,
    username: &str,
    old_password: &str,
    new_password: &str,
) -> Result<String, RustyError> {
    assert_same_user(cred, username)?;

    if let Some(mut user) = get_by_username(db, username).await? {
        if bcrypt::validate(old_password, &user.password)? {
            user.password = bcrypt::encode(new_password)?;
            db.update(USERS_INDEX, &user.id, &user.to_value()?).await?;
            Ok(user.id)
        } else {
            Err(RustyError::RequestError("password mismatch".to_string()))
        }
    } else {
        Err(RustyError::AsyncGraphqlError("user not found".to_string()))
    }
}

pub async fn update_preferences(
    db: &DbClient,
    cred: &Credential,
    username: &str,
    preferences: &str,
) -> Result<String, RustyError> {
    assert_same_user(cred, username)?;

    if let Some(mut user) = get_by_username(db, username).await? {
        user.preferences = Value::String(preferences.to_string());
        db.update(USERS_INDEX, &user.id, &user.to_value()?).await
    } else {
        Err(RustyError::AsyncGraphqlError("user not found".to_string()))
    }
}

pub async fn add_credential(
    db: &DbClient,
    sc: &ScClient,
    cred: &Credential,
    username: &str,
    input: &RegisterUserCredential,
) -> Result<String, RustyError> {
    assert_same_user(cred, username)?;

    if let Some(user) = get_by_username(db, username).await? {
        let credential = UserCredentialModel {
            id: UserCredential::generate_id(),
            name: input.name.to_string(),
            user_id: user.id,
        };
        let id = shared::create(db, CREDENTIALS_INDEX, input.clone(), credential).await?;
        sc.put(&id, &input.token).await?;
        Ok(id)
    } else {
        Err(RustyError::AsyncGraphqlError("user not found".to_string()))
    }
}

pub async fn delete_by_username(
    db: &DbClient,
    cred: &Credential,
    username: &str,
) -> Result<u64, RustyError> {
    assert_same_user(cred, username)?;

    if let Some(user) = get_by_username(db, username).await? {
        shared::delete_many(
            db,
            PERMISSIONS_INDEX,
            &json!({ "user_id": { "equals": user.id } }),
        )
        .await?;
        shared::delete_by_id(db, USERS_INDEX, &user.id).await
    } else {
        Err(RustyError::AsyncGraphqlError("user not found".to_string()))
    }
}

fn assert_same_user(cred: &Credential, username: &str) -> Result<(), RustyError> {
    let cred_username = get_username_claim(cred)?;
    if cred_username == username {
        Ok(())
    } else {
        Err(RustyError::UnauthorizedError)
    }
}
