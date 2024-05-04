use base64::Engine;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use commons::env::var;
use commons::errors::RustyError;

/// Server API for agents.
pub mod agents;

/// Server API for authentication.
pub mod auth;

/// Server API client wrapper.
pub mod client;

/// Server API for jobs.
pub mod jobs;

/// Server API for pipelines.
pub mod pipelines;

/// Server API for projects.
pub mod projects;

/// Utilities for Server API operations.
pub mod utils;

pub(crate) static JWT_TOKEN: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub(crate) fn get_credential() -> Result<String, RustyError> {
    let user = var::<String>("AGENT_USER")?;
    let pass = var::<String>("AGENT_PASSWORD")?;
    Ok(base64::prelude::BASE64_STANDARD.encode(format!("{user}:{pass}")))
}
