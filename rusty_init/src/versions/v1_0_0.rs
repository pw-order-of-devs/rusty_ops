use persist::db_client::DbClient;

use crate::ops::permissions::assign_permission;
use crate::ops::resources::create_resource;
use crate::ops::roles::create_role;
use crate::ops::schema::{execute_sql, purge_db};
use crate::ops::users::create_user;

pub async fn execute(db: &DbClient) {
    log::info!("=========================");
    log::info!("version v1.0.0 - starting");
    log::info!("=========================");

    // initialize db
    purge_db(db).await;
    execute_sql(db, "1.0.0").await;

    // create base users
    let admin_id = create_user(db, "admin").await;
    let agent_id = create_user(db, "agent").await;

    // create base roles
    let admins_role_id = create_role(db, "ADMINS", "System Administrators", &[&admin_id]).await;
    let agents_role_id = create_role(db, "AGENTS", "Agent System User", &[&agent_id]).await;
    let users_role_id = create_role(db, "USERS", "Standard User", &[]).await;

    // create system resources
    create_resource(db, "AGENTS", &["READ", "WRITE"]).await;
    create_resource(db, "JOBS", &["READ", "WRITE"]).await;
    create_resource(db, "PERMISSIONS", &["READ", "WRITE"]).await;
    create_resource(db, "PIPELINES", &["READ", "WRITE"]).await;
    create_resource(db, "PROJECTS", &["READ", "WRITE"]).await;
    create_resource(db, "USERS", &["READ", "WRITE"]).await;

    // assign permissions to admin role
    assign_permission(db, "AGENTS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "AGENTS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "JOBS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "JOBS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "PERMISSIONS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "PERMISSIONS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "PIPELINES", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "PIPELINES", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "PROJECT_GROUPS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "PROJECT_GROUPS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "PROJECTS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "PROJECTS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(db, "USERS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(db, "USERS", "WRITE", None, Some(&admins_role_id)).await;

    // assign permissions to agent role
    assign_permission(db, "AGENTS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(db, "AGENTS", "WRITE", None, Some(&agents_role_id)).await;
    assign_permission(db, "JOBS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(db, "PIPELINES", "READ", None, Some(&agents_role_id)).await;
    assign_permission(db, "PIPELINES", "WRITE", None, Some(&agents_role_id)).await;
    assign_permission(db, "PROJECT_GROUPS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(db, "PROJECTS", "READ", None, Some(&agents_role_id)).await;

    // assign permissions to user role
    assign_permission(db, "JOBS", "READ", None, Some(&users_role_id)).await;
    assign_permission(db, "PIPELINES", "READ", None, Some(&users_role_id)).await;
    assign_permission(db, "PROJECT_GROUPS", "READ", None, Some(&users_role_id)).await;
    assign_permission(db, "PROJECTS", "READ", None, Some(&users_role_id)).await;

    log::info!("=========================");
    log::info!("version v1.0.0 - done");
    log::info!("=========================");
}
