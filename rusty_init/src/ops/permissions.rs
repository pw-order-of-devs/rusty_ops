use serde_valid::json::json;

use persist::db_client::DbClient;

const PERMISSIONS_INDEX: &str = "permissions";

pub async fn assign_permission(
    db: &DbClient,
    resource: &str,
    right: &str,
    item: &str,
    user_id: Option<&str>,
    role_id: Option<&str>,
) {
    let message = format!("assigning `{resource}:{right}:{item}` to {user_id:?}/{role_id:?}");
    log::info!("{message}: start");

    let filter = if let Some(user_id) = user_id {
        json!({
            "user_id": { "equals": user_id },
            "resource": { "equals": resource },
            "right": { "equals": right },
            "item": { "equals": item },
        })
    } else if let Some(role_id) = role_id {
        json!({
            "role_id": { "equals": role_id },
            "resource": { "equals": resource },
            "right": { "equals": right },
            "item": { "equals": item },
        })
    } else {
        panic!("either `user_id` or `role_id` must be set");
    };

    if let Ok(None) = db.get_one(PERMISSIONS_INDEX, filter).await {
        match db
            .create(
                PERMISSIONS_INDEX,
                &json!({
                    "id": uuid::Uuid::new_v4().to_string(),
                    "user_id": user_id.map(ToString::to_string),
                    "role_id": role_id.map(ToString::to_string),
                    "resource": resource.to_string(),
                    "right": right.to_string(),
                    "item": item.to_string(),
                }),
            )
            .await
        {
            Ok(_) => log::info!("{message}: done"),
            Err(err) => panic!("{message}: `{err}`"),
        }
    } else {
        log::warn!("permission already granted: skipping");
    }
}
