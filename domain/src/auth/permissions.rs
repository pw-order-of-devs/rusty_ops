use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// A struct representing a Permission.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Permission {
    /// user assigned to permission
    pub user_id: Option<String>,
    /// role assigned to permission
    pub role_id: Option<String>,
    /// resource
    pub resource: String,
    /// right
    pub right: String,
}

impl RustyDomainItem for Permission {
    fn get_id(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.clone().user_id.unwrap_or_default(),
            self.clone().role_id.unwrap_or_default(),
            self.resource,
            self.right
        )
    }
}
