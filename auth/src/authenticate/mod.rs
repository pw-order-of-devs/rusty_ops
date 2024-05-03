use jwt::{Claims, VerifyWithKey};
use serde_json::json;

use commons::errors::RustyError;
use commons::hashing::bcrypt::validate;
use commons::hashing::sha::hmac512;
use domain::auth::credentials::get_token_username;
use domain::auth::user::User;
use persist::db_client::DbClient;

pub(crate) async fn basic_auth(
    db: &DbClient,
    user: &str,
    password: &str,
) -> Result<String, RustyError> {
    match db
        .get_one::<User>("users", json!({ "username": user }))
        .await?
    {
        Some(user) => {
            if validate(password, &user.password)? {
                Ok(user.username)
            } else {
                Err(RustyError::UnauthenticatedError)
            }
        }
        None => Err(RustyError::UnauthenticatedError),
    }
}

pub(crate) async fn bearer_auth(db: &DbClient, token: &str) -> Result<String, RustyError> {
    let user = get_token_username(token);
    match db
        .get_one::<User>("users", json!({ "username": user }))
        .await?
    {
        Some(user) => {
            let claims: Result<Claims, _> = token.verify_with_key(&hmac512(&user.password)?);
            match claims {
                Ok(_) => Ok(user.username),
                Err(_) => Err(RustyError::UnauthenticatedError),
            }
        }
        None => Err(RustyError::UnauthenticatedError),
    }
}
