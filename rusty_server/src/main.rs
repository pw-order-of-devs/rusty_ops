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
use poem::{listener::TcpListener, Route, Server};

mod gql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    commons::logger::init();
    let db = persist::init().await;

    // start the http server
    let app = Route::new()
        .at("/ws", GraphQL::new(gql::build_schema(db)));

    log::info!("Server is listening at: http://localhost:8000/ws");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
