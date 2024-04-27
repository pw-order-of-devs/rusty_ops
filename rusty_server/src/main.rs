//! `rusty_server` - server application for `rusty_ops`

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

use axum::{routing, Router};
use tokio::net::TcpListener;

use commons::env::var_or_default;

use crate::server_ext::{graphql_handler, graphql_ws_handler};

mod gql;
mod middleware;
mod schedulers;
mod server_ext;
mod services;

#[tokio::main]
async fn main() {
    commons::logger::init();
    let db = persist::init().await;
    schedulers::init(&db);
    let schema = gql::build_schema(&db);

    // start the http server
    let app = Router::new()
        .route("/health", routing::get(|| async { "ok" }))
        .route("/graphql", routing::post(graphql_handler))
        .route("/ws", routing::get(graphql_ws_handler))
        .layer(middleware::cors::cors_layer())
        .with_state(schema);

    let host = var_or_default("SERVER_ADDR", "0.0.0.0".to_string());
    let port = var_or_default("SERVER_PORT", "8000".to_string());
    let addr: std::net::SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Failed parsing server address");

    let listener = TcpListener::bind(addr).await.unwrap();
    log::info!("Server is listening at: :{port}/graphql");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    log::info!("Server is shut down");
}
