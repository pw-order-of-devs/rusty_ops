use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

use crate::RustyDomainItem;

/// A struct representing a project.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Project {
    /// project id
    pub id: String,
    /// project name
    pub name: String,
    /// project url
    pub url: Option<String>,
    /// project group id
    #[serde(rename(deserialize = "groupId", deserialize = "group_id"))]
    pub group_id: Option<String>,
}

/// A struct representing the registration of a project.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterProject {
    /// project name
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub name: String,
    /// project url
    #[validate(custom(validate_url))]
    pub url: String,
    /// project group id
    #[serde(rename(deserialize = "groupId", deserialize = "group_id"))]
    #[validate(min_length = 36)]
    #[validate(max_length = 36)]
    pub group_id: Option<String>,
}

fn validate_url(url: &str) -> Result<(), validation::Error> {
    match url::Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err(validation::Error::Custom("Invalid url".to_owned())),
    }
}

impl RegisterProject {
    /// constructor
    #[must_use]
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            group_id: None,
        }
    }
}

impl From<&RegisterProject> for Project {
    fn from(value: &RegisterProject) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
            url: Some(value.clone().url),
            group_id: value.clone().group_id,
        }
    }
}

impl RustyDomainItem for Project {
    fn get_id(&self) -> String {
        self.clone().id
    }
}

/// A struct representing a paged result Projects.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedProjects {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<Project>,
}
