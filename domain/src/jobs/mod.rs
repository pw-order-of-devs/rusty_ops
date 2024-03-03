use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RODomainItem;

/// A struct representing a job.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Job {
    id: String,
    description: Option<String>,
    project_id: String,
}

/// A struct representing the registration of a job.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterJob {
    description: Option<String>,
    project_id: String,
}

impl RODomainItem for Job {}
impl RODomainItem for RegisterJob {}
