use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::permissions::Permission;
use domain::auth::roles::Role;
use domain::auth::user::User;
use persist::db_client::DbClient;

pub(crate) async fn authorize(
    db: &DbClient,
    username: &str,
    resource: &str,
) -> Result<(), RustyError> {
    let user_id = get_user_id(db, username).await?;
    let mut permissions = get_permissions(db, &json!({ "user_id": user_id })).await?;
    let roles = get_user_roles_id(db, &user_id).await?;
    for role_id in roles {
        let p = get_permissions(db, &json!({ "role_id": role_id })).await?;
        permissions.extend_from_slice(&p);
    }
    if permissions
        .into_iter()
        .map(|res| format!("{}:{}", res.resource, res.right))
        .any(|x| x == *resource)
    {
        Ok(())
    } else {
        Err(RustyError::UnauthenticatedError)
    }
}

async fn get_user_id(db: &DbClient, username: &str) -> Result<String, RustyError> {
    match db
        .get_one::<User>("users", json!({ "username": username }))
        .await?
    {
        Some(user) => Ok(user.id),
        None => Err(RustyError::RequestError("User was not found".to_string())),
    }
}

async fn get_user_roles_id(db: &DbClient, user_id: &str) -> Result<Vec<String>, RustyError> {
    let roles = db
        .get_all::<Role>("roles", &None, &None, false)
        .await?
        .into_iter()
        .filter(|role| role.users.contains(&user_id.to_string()))
        .map(|role| role.id)
        .collect::<Vec<String>>();
    Ok(roles)
}

async fn get_permissions(db: &DbClient, filter: &Value) -> Result<Vec<Permission>, RustyError> {
    db.get_all("permissions", &Some(filter.clone()), &None, false)
        .await
}
