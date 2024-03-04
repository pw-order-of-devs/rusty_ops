use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

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

impl From<&RegisterJob> for Job {
    fn from(value: &RegisterJob) -> Self {
        Self {
            id: Self::generate_id(),
            description: value.clone().description,
            project_id: value.clone().project_id,
        }
    }
}

impl RustyDomainItem for Job {

    fn id(&self) -> String {
        self.clone().id
    }
}

impl RustyDomainItem for RegisterJob {

    fn id(&self) -> String {
        todo!()
    }
}
