use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::projects::Project;
use crate::RustyDomainItem;

/// A struct representing a project group.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Group {
    /// group id
    pub id: String,
    /// group name
    pub name: String,
    /// group projects
    pub projects: Vec<Project>,
}

/// A struct representing the registration of a project group.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterGroup {
    /// project group name
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub name: String,
}

impl RegisterGroup {
    /// constructor
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl From<&RegisterGroup> for Group {
    fn from(value: &RegisterGroup) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
            projects: vec![],
        }
    }
}

impl RustyDomainItem for Group {
    fn get_id(&self) -> String {
        self.clone().id
    }
}

/// A struct representing a paged result Projects.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedGroups {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<Group>,
}
