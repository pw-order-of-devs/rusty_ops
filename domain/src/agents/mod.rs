use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::RustyDomainItem;

/// A struct representing a job.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Agent {
    /// agent id
    pub id: String,
    /// agent expiry timestamp in UTC
    pub expiry: i64,
}

impl Agent {
    /// update agent's expiry time
    pub fn update_expiry(&mut self, ttl: i64) {
        let agent_ttl = ttl;
        self.expiry = chrono::Utc::now().timestamp() + agent_ttl;
    }

    /// Convert `RegisterAgent` into `Agent`.
    #[must_use]
    pub fn from(value: &RegisterAgent, ttl: i64) -> Self {
        let agent_ttl = ttl;
        Self {
            id: value.clone().id,
            expiry: chrono::Utc::now().timestamp() + agent_ttl,
        }
    }
}

/// A struct representing the registration of an agent.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterAgent {
    /// agent id
    #[validate(min_length = 36)]
    #[validate(max_length = 36)]
    pub id: String,
}

impl RustyDomainItem for Agent {}

/// A struct representing a paged result Agents.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedAgents {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<Agent>,
}
