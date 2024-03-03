use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RODomainItem;

/// S struct representing a pipeline.
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

/// A struct representing the registration of a pipeline.
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
