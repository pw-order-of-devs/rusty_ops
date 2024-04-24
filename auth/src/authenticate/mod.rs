use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;

use serde_json::json;

/// authenticate user using his credential
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authenticate(db: &DbClient, credential: &Credential) -> Result<User, RustyError> {
    match credential {
        Credential::Basic(user, pass) => basic_auth(db, user, pass).await,
    }
}

async fn basic_auth(db: &DbClient, user: &str, password: &str) -> Result<User, RustyError> {
    let user = db
        .get_one("users", json!({ "username": user, "password": password }))
        .await?;
    user.ok_or(RustyError::UserNotFoundError)
}
