use serde_json::json;

use commons::errors::RustyError;
use commons::hashing::bcrypt::validate;
use domain::auth::user::User;
use persist::db_client::DbClient;

pub(crate) async fn basic_auth(
    db: &DbClient,
    user: &str,
    password: &str,
) -> Result<Option<User>, RustyError> {
    match db
        .get_one::<User>("users", json!({ "username": user }))
        .await?
    {
        Some(user) => {
            if validate(password, &user.password)? {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}
