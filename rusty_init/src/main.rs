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
use rusty_init::{ops, versions};

#[tokio::main]
async fn main() {
    commons::logger::init();
    let db = get_db_client().await;

    // clear database if WIPE_DATA set to true
    ops::schema::purge_db(&db).await;

    // v1.0.0
    versions::v1_0_0::execute(&db).await;
}

async fn get_db_client() -> DbClient {
    let db_type = commons::env::var_or_default("RUSTY_PERSISTENCE", String::new());
    log::info!("initializing database: {db_type}");
    persist::init().await
}
