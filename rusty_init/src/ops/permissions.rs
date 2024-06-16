use domain::auth::permissions::Permission;
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
    log::info!("assigning `{resource}:{right}{item}` to {user_id:?}/{role_id:?} permission: start");
    let permission = Permission {
        user_id: user_id.map(ToString::to_string),
        role_id: role_id.map(ToString::to_string),
        resource: resource.to_string(),
        right: right.to_string(),
        item: item.to_string(),
    };
    match db.create(PERMISSIONS_INDEX, &permission).await {
        Ok(_) => {
            log::info!(
                "assigning `{resource}:{right}` to {user_id:?}/{role_id:?} permission: done"
            );
        }
        Err(err) => {
            panic!("assigning `{resource}:{right}` to {user_id:?}/{role_id:?} permission: `{err}`");
        }
    }
}
