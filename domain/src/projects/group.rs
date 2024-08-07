use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::projects::ProjectModel;
use crate::RustyDomainItem;

/// A struct representing a project group.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct GroupModel {
    /// group id
    pub id: String,
    /// group name
    pub name: String,
    /// group projects
    pub projects: Vec<ProjectModel>,
}

/// A struct representing a project group.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Group {
    /// group id
    pub id: String,
    /// group name
    pub name: String,
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

impl From<&Group> for GroupModel {
    fn from(value: &Group) -> Self {
        Self {
            id: value.clone().id,
            name: value.clone().name,
            projects: vec![],
        }
    }
}

impl From<&RegisterGroup> for Group {
    fn from(value: &RegisterGroup) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
        }
    }
}

impl RustyDomainItem for GroupModel {}

impl RustyDomainItem for Group {}

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
    pub entries: Vec<GroupModel>,
}
