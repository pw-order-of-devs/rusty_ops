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
    /// job pipeline template
    pub template: String,
    /// job project id
    #[serde(rename(deserialize = "projectId", deserialize = "project_id"))]
    pub project_id: String,
}

/// A struct representing the registration of a job.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterJob {
    /// job name
    pub name: String,
    /// job description
    pub description: Option<String>,
    /// job pipeline template
    pub template: String,
    /// job project id
    #[serde(rename(deserialize = "projectId", deserialize = "project_id"))]
    pub project_id: String,
}

impl RegisterJob {
    /// constructor
    #[must_use]
    pub fn new(name: &str, description: &str, template: &str, project_id: &str) -> Self {
        Self {
            name: name.to_string(),
            description: if description.is_empty() {
                None
            } else {
                Some(description.to_string())
            },
            template: template.to_string(),
            project_id: project_id.to_string(),
        }
    }
}

impl From<&RegisterJob> for Job {
    fn from(value: &RegisterJob) -> Self {
        Self {
            id: Self::generate_id(),
            name: value.clone().name,
            description: value.clone().description,
            template: value.clone().template,
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
