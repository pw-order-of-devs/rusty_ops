use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

use crate::jobs::JobModel;
use crate::RustyDomainItem;

/// An enum representing a project source (internal or from repository).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum Source {
    /// project hosted internally
    Internal,
    /// project referencing GitHub
    GitHub,
}

/// A struct representing an internal project.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct ProjectModel {
    /// project id
    pub id: String,
    /// project source
    pub source: Source,
    /// project name
    pub name: String,
    /// project url
    pub url: String,
    /// project main branch name
    #[serde(rename(deserialize = "mainBranch", deserialize = "main_branch"))]
    pub main_branch: String,
    /// project group id
    #[serde(rename(deserialize = "groupId", deserialize = "group_id"))]
    pub group_id: Option<String>,
    /// project jobs
    pub jobs: Vec<JobModel>,
}

/// A struct representing a project.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Project {
    /// project id
    pub id: String,
    /// project source
    pub source: Source,
    /// project name
    pub name: Option<String>,
    /// project url
    pub url: Option<String>,
    /// project main branch name
    #[serde(rename(deserialize = "mainBranch", deserialize = "main_branch"))]
    pub main_branch: Option<String>,
    /// project group id
    #[serde(rename(deserialize = "groupId", deserialize = "group_id"))]
    pub group_id: Option<String>,
}

/// A struct representing the registration of a project.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterProject {
    /// project source
    pub source: Source,
    /// project name
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub name: Option<String>,
    /// project url
    #[validate(custom(validate_url))]
    pub url: Option<String>,
    /// project main branch name
    #[serde(rename(deserialize = "mainBranch", deserialize = "main_branch"))]
    pub main_branch: Option<String>,
    /// project group id
    #[serde(rename(deserialize = "groupId", deserialize = "group_id"))]
    #[validate(min_length = 36)]
    #[validate(max_length = 36)]
    pub group_id: Option<String>,
}

fn validate_url(url: &Option<String>) -> Result<(), validation::Error> {
    url.as_ref().map_or_else(
        || Ok(()),
        |url| match url::Url::parse(url) {
            Ok(_) => Ok(()),
            Err(_) => Err(validation::Error::Custom("Invalid url".to_owned())),
        },
    )
}

impl RegisterProject {
    /// constructor
    #[must_use]
    pub fn new(source: &Source, name: &Option<String>, url: &str) -> Self {
        Self {
            source: *source,
            name: name.clone(),
            url: Some(url.to_string()),
            main_branch: Some("master".to_string()),
            group_id: None,
        }
    }
}

impl From<&Project> for ProjectModel {
    fn from(value: &Project) -> Self {
        Self {
            id: value.clone().id,
            source: value.clone().source,
            name: value.clone().name.unwrap_or_default(),
            url: value.clone().url.unwrap_or_default(),
            main_branch: value.clone().main_branch.unwrap_or_default(),
            group_id: value.clone().group_id,
            jobs: vec![],
        }
    }
}

impl From<&RegisterProject> for Project {
    fn from(value: &RegisterProject) -> Self {
        Self {
            id: Self::generate_id(),
            source: value.clone().source,
            name: value.clone().name,
            url: value.clone().url,
            main_branch: value.clone().main_branch,
            group_id: value.clone().group_id,
        }
    }
}

impl RustyDomainItem for ProjectModel {}

impl RustyDomainItem for Project {}

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
    pub entries: Vec<ProjectModel>,
}
