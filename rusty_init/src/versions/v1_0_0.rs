use persist::db_client::DbClient;

use crate::ops::permissions::assign_permission;
use crate::ops::resources::create_resource;
use crate::ops::roles::create_role;
use crate::ops::schema::{execute_sql, purge_db};
use crate::ops::users::create_user;
use crate::ops::versions;

pub async fn execute(db: &DbClient) {
    let version = "1.0.0";
    if versions::is_installed(db, version).await {
        log::info!("=========================");
        log::info!("version v{version} - already installed - skipping");
        log::info!("=========================");
    } else {
        log::info!("=========================");
        log::info!("version v{version} - starting");
        log::info!("=========================");

        // initialize db
        purge_db(db).await;
        execute_sql(db, version).await;

        // create system resources
        create_resource(db, "AGENTS", &["READ", "WRITE"]).await;
        create_resource(db, "AUTH", &["READ", "WRITE"]).await;
        create_resource(db, "PROJECT_GROUPS", &["CREATE", "READ", "WRITE"]).await;
        create_resource(db, "PROJECTS", &["CREATE", "READ", "WRITE"]).await;
        create_resource(db, "USERS", &["READ", "WRITE"]).await;

        // create admin user
        if let Some(id) = create_user(db, "admin").await {
            // create admins role
            if let Some(role_id) = create_role(db, "ADMINS", "System Administrators", &[&id]).await
            {
                // assign permissions to admin role
                assign_permission(db, "AGENTS", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "AGENTS", "WRITE", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "AUTH", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "AUTH", "WRITE", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECT_GROUPS", "CREATE", "ALL", None, Some(&role_id))
                    .await;
                assign_permission(db, "PROJECT_GROUPS", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECT_GROUPS", "WRITE", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECTS", "CREATE", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECTS", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECTS", "WRITE", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "USERS", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "USERS", "WRITE", "ALL", None, Some(&role_id)).await;
            }
        }

        // create agent user
        if let Some(id) = create_user(db, "agent").await {
            // create agents role
            if let Some(role_id) = create_role(db, "AGENTS", "Agent System User", &[&id]).await {
                // assign permissions to agent role
                assign_permission(
                    db,
                    "AGENTS",
                    "READ",
                    &format!("ID[{role_id}]"),
                    None,
                    Some(&role_id),
                )
                .await;
                assign_permission(
                    db,
                    "AGENTS",
                    "WRITE",
                    &format!("ID[{role_id}]"),
                    None,
                    Some(&role_id),
                )
                .await;
                assign_permission(db, "PROJECTS", "READ", "ALL", None, Some(&role_id)).await;
                assign_permission(db, "PROJECTS", "WRITE", "ALL", None, Some(&role_id)).await;
            }
        }

        // create roles
        let _ = create_role(db, "USERS", "Standard User", &[]).await;
        versions::insert(db, version).await;

        log::info!("=========================");
        log::info!("version v{version} - done");
        log::info!("=========================");
    }
}
