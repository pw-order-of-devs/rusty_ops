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

use async_graphql_poem::{GraphQL, GraphQLSubscription};
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

mod gql;
mod middleware;

#[handler]
fn health() -> String {
    "ok".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    commons::logger::init();
    let db = persist::init().await;
    let schema = gql::build_schema(db.clone());

    // start the http server
    let app = Route::new()
        .at("/health", get(health))
        .at("/graphql", GraphQL::new(schema.clone()))
        .at("/ws", get(GraphQLSubscription::new(schema)))
        .with(middleware::cors::cors_config());

    let host = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());
    log::info!("Server is listening at: :{port}/graphql");
    Server::new(TcpListener::bind(format!("{host}:{port}")))
        .run(app)
        .await?;
    Ok(())
}
