use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// A struct representing a Role.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Role {
    /// role id
    pub id: String,
    /// role name
    pub name: String,
    /// role description
    pub description: Option<String>,
    /// assigned users' ids
    pub users: Vec<String>,
}

impl RustyDomainItem for Role {
    fn get_id(&self) -> String {
        self.clone().id
    }
}
