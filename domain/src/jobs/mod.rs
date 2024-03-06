use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// A struct representing a job.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Job {
    /// job id
    pub id: String,
    /// job name
    pub name: String,
    /// job description
    pub description: Option<String>,
    /// job project id
    #[serde(rename(deserialize = "projectId", deserialize = "project_id"))]
    pub project_id: String,
}

/// A struct representing the registration of a job.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterJob {
    name: String,
    description: Option<String>,
    #[serde(rename(deserialize = "projectId", deserialize = "project_id"))]
    project_id: String,
}

impl From<&RegisterJob> for Job {
    fn from(value: &RegisterJob) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
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
