use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::jobs::Job;
use crate::RODomainItem;

/// A struct representing a project.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Project {
    id: String,
    url: String,
    jobs: Vec<Job>,
}

/// A struct representing the registration of a project.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterProject {
    url: String,
}

impl RODomainItem for Project {}
impl RODomainItem for RegisterProject {}
