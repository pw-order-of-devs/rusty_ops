use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// A struct representing a User.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct User {
    /// user id
    pub id: String,
    /// username
    pub username: String,
    /// username
    pub password: String,
}

impl RustyDomainItem for User {
    fn id(&self) -> String {
        self.clone().id
    }
}
