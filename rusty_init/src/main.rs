//! `rusty_init` - database initialization for `rusty_ops`

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

use persist::db_client::DbClient;

use crate::ops::permissions::assign_permission;
use crate::ops::resources::create_resource;
use crate::ops::{
    roles::create_role,
    schema::{create, purge_db},
    users::create_user,
};

mod ops;

#[tokio::main]
async fn main() {
    commons::logger::init();
    let db = get_db_client().await;

    // initialize db
    purge_db(&db).await;
    create(&db).await;

    // create base users
    let admin_id = create_user(&db, "admin").await;
    let agent_id = create_user(&db, "agent").await;

    // create base roles
    let admins_role_id = create_role(&db, "ADMINS", "System Administrators", &[&admin_id]).await;
    let agents_role_id = create_role(&db, "AGENTS", "Agent System User", &[&agent_id]).await;
    let users_role_id = create_role(&db, "USERS", "Standard User", &[]).await;

    // create system resources
    create_resource(&db, "AGENTS", &["READ", "WRITE"]).await;
    create_resource(&db, "JOBS", &["READ", "WRITE"]).await;
    create_resource(&db, "PERMISSIONS", &["READ", "WRITE"]).await;
    create_resource(&db, "PIPELINES", &["READ", "WRITE"]).await;
    create_resource(&db, "PROJECTS", &["READ", "WRITE"]).await;
    create_resource(&db, "USERS", &["READ", "WRITE"]).await;

    // assign permissions to admin role
    assign_permission(&db, "AGENTS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "AGENTS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "JOBS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "JOBS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PERMISSIONS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PERMISSIONS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PIPELINES", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PIPELINES", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PROJECT_GROUPS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PROJECT_GROUPS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PROJECTS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "PROJECTS", "WRITE", None, Some(&admins_role_id)).await;
    assign_permission(&db, "USERS", "READ", None, Some(&admins_role_id)).await;
    assign_permission(&db, "USERS", "WRITE", None, Some(&admins_role_id)).await;

    // assign permissions to agent role
    assign_permission(&db, "AGENTS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(&db, "AGENTS", "WRITE", None, Some(&agents_role_id)).await;
    assign_permission(&db, "JOBS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(&db, "PIPELINES", "READ", None, Some(&agents_role_id)).await;
    assign_permission(&db, "PIPELINES", "WRITE", None, Some(&agents_role_id)).await;
    assign_permission(&db, "PROJECT_GROUPS", "READ", None, Some(&agents_role_id)).await;
    assign_permission(&db, "PROJECTS", "READ", None, Some(&agents_role_id)).await;

    // assign permissions to user role
    assign_permission(&db, "JOBS", "READ", None, Some(&users_role_id)).await;
    assign_permission(&db, "PIPELINES", "READ", None, Some(&users_role_id)).await;
    assign_permission(&db, "PROJECT_GROUPS", "READ", None, Some(&users_role_id)).await;
    assign_permission(&db, "PROJECTS", "READ", None, Some(&users_role_id)).await;
}

async fn get_db_client() -> DbClient {
    let db_type = commons::env::var_or_default("RUSTY_PERSISTENCE", String::new());
    log::info!("initializing database: {db_type}");
    persist::init().await
}
