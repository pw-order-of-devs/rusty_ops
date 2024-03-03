use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RODomainItem;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Job {
    id: String,
    description: Option<String>,
    project_id: String,
}

#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterJob {
    description: Option<String>,
    project_id: String,
}

impl RODomainItem for Job {}
impl RODomainItem for RegisterJob {}
