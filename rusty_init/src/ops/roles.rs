use serde_valid::json::json;

use persist::db_client::DbClient;

const ROLES_INDEX: &str = "roles";

pub async fn create_role(
    db: &DbClient,
    name: &str,
    description: &str,
    users: &[&str],
) -> Option<String> {
    log::info!("creating `{name}` role: start");
    let role = json!({
        "id": uuid::Uuid::new_v4().to_string(),
        "name": name.to_string(),
        "description": Some(description.to_string()),
        "users": users.iter().map(|it| (*it).to_string()).collect::<Vec<String>>(),
    });
    if let Ok(Some(_)) = db
        .get_one(ROLES_INDEX, json!({ "name": { "equals": name } }))
        .await
    {
        log::warn!("role `{name}` already exists - skipping");
        None
    } else {
        match db.create(ROLES_INDEX, &role).await {
            Ok(id) => {
                log::info!("creating `{name}` role: done");
                Some(id)
            }
            Err(err) => panic!("error while creating role `{name}`: `{err}`"),
        }
    }
}
