use serde_json::json;

use commons::errors::RustyError;
use commons::hashing::sha512;
use domain::auth::user::User;
use persist::db_client::DbClient;

pub(crate) async fn basic_auth(
    db: &DbClient,
    user: &str,
    password: &str,
) -> Result<Option<User>, RustyError> {
    let pass_sha512 = sha512(password);
    db.get_one::<User>(
        "users",
        json!({ "username": user, "password": pass_sha512 }),
    )
    .await
}
