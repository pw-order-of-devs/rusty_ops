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

use std::error::Error;

use async_graphql_poem::GraphQL;
use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::http::Method;
use poem::middleware::Cors;

mod gql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    commons::logger::init();
    let db = persist::init().await;

    let cors = Cors::new()
        .allow_methods(vec![Method::POST, Method::OPTIONS])
        .allow_origin("http://localhost:8080")
        .allow_header("content-type")
        .allow_credentials(true);

    // start the http server
    let app = Route::new()
        .at("/graphql", GraphQL::new(gql::build_schema(db)))
        .with(cors);

    log::info!("Server is listening at: :8000/graphql");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
