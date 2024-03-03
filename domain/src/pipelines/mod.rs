use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RODomainItem;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Pipeline {
    id: String,
    name: String,
    // conditions: Vec<Condition>,
    // shared: Shared,
    // conditions: Vec<Condition>,
    // stages: Vec<Stage>,
    job_id: String,
}

#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterPipeline {
    name: String,
    // conditions: Vec<Condition>,
    // shared: Shared,
    // stages: Vec<Stage>,
    job_id: String,
}

impl RODomainItem for Pipeline {}
impl RODomainItem for RegisterPipeline {}
