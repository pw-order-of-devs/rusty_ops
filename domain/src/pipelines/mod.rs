use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

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

impl RustyDomainItem for Pipeline {

    fn id(&self) -> String {
        self.clone().id
    }
}

impl RustyDomainItem for RegisterPipeline {

    fn id(&self) -> String {
        todo!()
    }
}
