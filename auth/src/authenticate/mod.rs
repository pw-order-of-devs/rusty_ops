use commons::errors::RustyError;
use domain::auth::user::User;
use persist::db_client::DbClient;

use serde_json::json;

pub(crate) async fn basic_auth(
    db: &DbClient,
    user: &str,
    password: &str,
) -> Result<User, RustyError> {
    let user = db
        .get_one("users", json!({ "username": user, "password": password }))
        .await?;
    user.ok_or(RustyError::UserNotFoundError)
}
