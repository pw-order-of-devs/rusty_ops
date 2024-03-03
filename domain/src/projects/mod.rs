use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::jobs::Job;
use crate::RODomainItem;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Project {
    id: String,
    url: String,
    jobs: Vec<Job>,
}

#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterProject {
    url: String,
}

impl RODomainItem for Project {}
impl RODomainItem for RegisterProject {}
