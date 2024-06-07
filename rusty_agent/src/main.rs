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

use axum::{routing, Router};
use tokio::net::TcpListener;

use commons::env::var_or_default;
use rusty_agent::{api, resolver};

#[tokio::main]
async fn main() {
    let uuid = init().await;

    // start the http server
    let app = Router::new().route("/health", routing::get(|| async { "ok" }));

    let host = var_or_default("AGENT_ADDR", "0.0.0.0".to_string());
    let port = var_or_default("AGENT_PORT", "8800".to_string());
    let addr: std::net::SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Failed parsing server address");

    let listener = TcpListener::bind(addr).await.unwrap();
    log::info!("Agent is listening at: :{port}/graphql");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");

    let _ = api::agents::unregister(&uuid).await;
    log::info!("Server is shut down");
}

async fn init() -> String {
    commons::logger::init();

    let uuid = uuid::Uuid::new_v4().to_string();
    let token = api::auth::authenticate()
        .await
        .expect("Failed to authenticate agent");
    *api::JWT_TOKEN.lock().unwrap() = token;
    api::agents::register(&uuid)
        .await
        .expect("Error while registering the agent");
    resolver::init(&uuid);
    log::debug!("Initialized with id: `{uuid}`");
    uuid
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
