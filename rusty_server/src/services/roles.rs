use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::roles::Role;
use domain::RustyDomainItem;
use persist::db_client::DbClient;

use crate::services::{shared, users};

const ROLES_INDEX: &str = "roles";

// query

async fn get_one(db: &DbClient, value: &Value) -> Result<Option<Role>, RustyError> {
    shared::get_one(db, ROLES_INDEX, value).await
}

// mutate

pub async fn assign(
    db: &DbClient,
    cred: &Credential,
    user_id: &str,
    role_id: Option<&str>,
    role_name: Option<&str>,
) -> Result<String, RustyError> {
    if users::get_by_id(db, cred, user_id).await?.is_some() {
        let doc = match (role_id, role_name) {
            (Some(id), None) => json!({ "id": { "equals": id } }),
            (None, Some(name)) => json!({ "name": { "equals": name } }),
            (_, _) => {
                let message =
                    "`roles::assign` - one of `role_id` or `role_name` must be filled".to_string();
                log::debug!("{message}");
                return Err(RustyError::AsyncGraphqlError(message));
            }
        };

        if let Some(mut role) = get_one(db, &doc).await? {
            role.users.push(user_id.to_string());
            let _ = db.update(ROLES_INDEX, &role.id, &role.to_value()?).await?;
            Ok(role.id)
        } else {
            let message = "`roles::assign` - role not found".to_string();
            log::debug!("{message}");
            Err(RustyError::AsyncGraphqlError(message))
        }
    } else {
        let message = "`roles::assign` - user not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}
