use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// A struct representing a Permission.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Resource {
    /// resource id
    pub id: String,
    /// resource name
    pub name: String,
    /// resource rights
    pub rights: Vec<String>,
}

impl RustyDomainItem for Resource {
    fn get_id(&self) -> String {
        self.clone().name
    }
}
