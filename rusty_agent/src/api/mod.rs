use base64::Engine;

use commons::env::var;
use commons::errors::RustyError;

/// Server API for agents.
pub mod agents;

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

pub(crate) fn get_credential() -> Result<String, RustyError> {
    let user = var::<String>("AGENT_USER")?;
    let pass = var::<String>("AGENT_PASSWORD")?;
    let credential = format!("{user}:{pass}");
    let credential = base64::prelude::BASE64_STANDARD.encode(credential);
    Ok(format!("Basic {credential}"))
}
