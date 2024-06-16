use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

use crate::RustyDomainItem;

/// A struct representing a Permission.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize, Validate)]
pub struct Permission {
    /// user assigned to permission
    pub user_id: Option<String>,
    /// role assigned to permission
    pub role_id: Option<String>,
    /// resource
    pub resource: String,
    /// right
    pub right: String,
    /// item
    #[validate(custom(validate_item))]
    pub item: String,
}

fn validate_item(item: &str) -> Result<(), validation::Error> {
    let re = regex::Regex::new(r"^ID\[[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}]$").unwrap();
    if item == "ALL" || re.is_match(item) {
        Ok(())
    } else {
        Err(validation::Error::Custom(
            "not supported item type for permission".to_string(),
        ))
    }
}

impl Permission {
    /// constructor
    #[must_use]
    pub fn new(
        user_id: Option<String>,
        role_id: Option<String>,
        resource: &str,
        right: &str,
        item: &str,
    ) -> Self {
        Self {
            user_id,
            role_id,
            resource: resource.to_string(),
            right: right.to_string(),
            item: item.to_string(),
        }
    }
}

impl RustyDomainItem for Permission {
    fn get_id(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.clone().user_id.unwrap_or_default(),
            self.clone().role_id.unwrap_or_default(),
            self.resource,
            self.right,
            self.item,
        )
    }
}
