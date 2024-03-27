use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use commons::env::var_or_default;

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
    pub fn update_expiry(&mut self) {
        let agent_ttl = var_or_default("AGENT_TTL", 300);
        self.expiry = chrono::Utc::now().timestamp() + agent_ttl;
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

impl From<&RegisterAgent> for Agent {
    fn from(value: &RegisterAgent) -> Self {
        let agent_ttl = var_or_default("AGENT_TTL", 300);
        Self {
            id: value.clone().id,
            expiry: chrono::Utc::now().timestamp() + agent_ttl,
        }
    }
}

impl RustyDomainItem for Agent {
    fn id(&self) -> String {
        self.clone().id
    }
}

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
