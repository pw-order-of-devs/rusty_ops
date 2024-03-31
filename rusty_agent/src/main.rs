//! `rusty_agent` - agent application for `rusty_ops`

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

use poem::{get, handler, listener::TcpListener, Route, Server};

use commons::env::var_or_default;

mod api;

/// scheduler for resolving pipelines
mod resolver;

#[handler]
fn health() -> String {
    "ok".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    commons::logger::init();

    let uuid = uuid::Uuid::new_v4().to_string();
    api::agents::register(&uuid)
        .await
        .expect("Error while registering the agent");
    resolver::init(uuid.clone());
    log::debug!("Initialized with id: `{uuid}`");

    // start the http server
    let app = Route::new().at("/health", get(health));

    let host = var_or_default("AGENT_ADDR", "0.0.0.0".to_string());
    let port = var_or_default("AGENT_PORT", "8800".to_string());
    log::info!("Agent is listening at: :{port}/graphql");
    Server::new(TcpListener::bind(format!("{host}:{port}")))
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(std::time::Duration::from_secs(5)),
        )
        .await?;

    let _ = api::agents::unregister(&uuid).await;
    Ok(())
}
