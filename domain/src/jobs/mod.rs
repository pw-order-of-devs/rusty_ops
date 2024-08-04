use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

use crate::pipelines::Pipeline;
use crate::templates::pipeline::PipelineTemplate;
use crate::RustyDomainItem;

/// A struct representing a job.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct JobModel {
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
    /// job pipelines
    pub pipelines: Vec<Pipeline>,
}

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
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterJob {
    /// job name
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub name: String,
    /// job description
    #[validate(max_length = 2048)]
    pub description: Option<String>,
    /// job pipeline template
    #[validate(custom(validate_template))]
    pub template: String,
    /// job project id
    #[serde(rename(deserialize = "projectId", deserialize = "project_id"))]
    #[validate(min_length = 36)]
    #[validate(max_length = 36)]
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

fn validate_template(url: &str) -> Result<(), validation::Error> {
    match PipelineTemplate::from_yaml(url) {
        Ok(_) => Ok(()),
        Err(_) => Err(validation::Error::Custom(
            "Invalid pipeline template".to_owned(),
        )),
    }
}

impl From<&Job> for JobModel {
    fn from(value: &Job) -> Self {
        Self {
            id: value.clone().id,
            name: value.clone().name,
            description: value.clone().description,
            template: value.clone().template,
            project_id: value.clone().project_id,
            pipelines: vec![],
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

impl RustyDomainItem for JobModel {}

impl RustyDomainItem for Job {}

/// A struct representing a paged result Jobs.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedJobs {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<JobModel>,
}
