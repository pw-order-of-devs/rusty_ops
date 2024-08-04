//! Auth module for `rusty_ops`

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, deny(rust_2018_idioms))]

use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::{parse_credential, Credential};
use domain::auth::permissions::Permission;
use domain::auth::roles::Role;
use persist::db_client::DbClient;

/// authenticate user
pub mod authenticate;

/// authorize user
pub mod authorize;

/// jwt token operations
pub mod token;

/// Parses an authentication header and returns a credential if the header is valid.
///
/// # Arguments
///
/// * `header` - value of Authentication header
///
/// # Returns
///
/// * `Some(Credential)` if the header is valid and represents a basic authentication with a username and password.
/// * `None` if the header is invalid or does not represent a basic authentication.
#[must_use]
pub fn parse_auth_header(header: &str) -> Credential {
    let value = header.split(' ').collect::<Vec<&str>>();
    if value.len() != 2 {
        log::warn!("malformed auth header");
        return Credential::None;
    }
    parse_credential(value[0], value[1])
}

/// authenticate user using his credential
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authenticate(db: &DbClient, credential: &Credential) -> Result<String, RustyError> {
    match credential {
        Credential::Basic(user, pass) => authenticate::basic_auth(db, user, pass).await,
        Credential::Bearer(token) => authenticate::bearer_auth(db, token).await,
        Credential::None | Credential::System => Ok(String::new()),
    }
}

/// authenticate user for permission
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authorize(db: &DbClient, username: &str, resources: &str) -> Result<(), RustyError> {
    authorize::authorize(db, username, resources).await
}

/// fetch list of permissions for user
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn get_user_permissions(
    db: &DbClient,
    username: &str,
) -> Result<Vec<Permission>, RustyError> {
    let user_id = get_user_id(db, username).await?;
    let mut permissions = get_permissions(db, &json!({ "user_id": { "equals": user_id } })).await?;
    let roles = get_user_roles_id(db, &user_id).await?;
    for role_id in roles {
        let p = get_permissions(db, &json!({ "role_id": { "equals": role_id } })).await?;
        permissions.extend_from_slice(&p);
    }
    Ok(permissions)
}

async fn get_user_id(db: &DbClient, username: &str) -> Result<String, RustyError> {
    db.get_one("users", json!({ "username": { "equals": username } }))
        .await?
        .map_or_else(
            || Err(RustyError::RequestError("User was not found".to_string())),
            |user| {
                Ok(user
                    .get("id")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or_default()
                    .to_string())
            },
        )
}

async fn get_user_roles_id(db: &DbClient, user_id: &str) -> Result<Vec<String>, RustyError> {
    let roles = db
        .get_all("roles", &None, &None)
        .await?
        .into_iter()
        .filter_map(|v| serde_json::from_value::<Role>(v).ok())
        .filter(|role| role.users.contains(&user_id.to_string()))
        .map(|role| role.id)
        .collect::<Vec<String>>();
    Ok(roles)
}

async fn get_permissions(db: &DbClient, filter: &Value) -> Result<Vec<Permission>, RustyError> {
    let values = db
        .get_all("permissions", &Some(filter.clone()), &None)
        .await?
        .into_iter()
        .filter_map(|v| serde_json::from_value::<Permission>(v).ok())
        .collect();
    Ok(values)
}
