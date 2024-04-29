use serde_json::json;

use commons::errors::RustyError;
use commons::hashing::bcrypt::validate;
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
