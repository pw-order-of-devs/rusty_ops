use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::RustyDomainItem;

/// An enum representing a pipeline status.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum PipelineStatus {
    /// Created pipeline, waiting for agent to be assigned to.
    #[serde(rename(deserialize = "DEFINED", deserialize = "Defined"))]
    Defined,
    /// Pipeline assigned to an agent, not yet started.
    #[serde(rename(deserialize = "ASSIGNED", deserialize = "Assigned"))]
    Assigned,
    /// Currently running pipeline.
    #[serde(rename(deserialize = "IN_PROGRESS", deserialize = "InProgress"))]
    InProgress,
    /// Pipeline finished successfully.
    #[serde(rename(deserialize = "SUCCESS", deserialize = "Success"))]
    Success,
    /// Pipeline finished with a failure.
    #[serde(rename(deserialize = "FAILURE", deserialize = "Failure"))]
    Failure,
    /// Pipeline finished in an unstable state.
    #[serde(rename(deserialize = "UNSTABLE", deserialize = "Unstable"))]
    Unstable,
}

/// A struct representing a pipeline.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Pipeline {
    /// pipeline id
    pub id: String,
    /// pipeline order number
    pub number: u64,
    /// pipeline branch
    pub branch: String,
    /// pipeline register date
    #[serde(rename(deserialize = "registerDate", deserialize = "register_date"))]
    pub register_date: String,
    /// pipeline start date
    #[serde(rename(deserialize = "startDate", deserialize = "start_date"))]
    pub start_date: Option<String>,
    /// pipeline end date
    #[serde(rename(deserialize = "endDate", deserialize = "end_date"))]
    pub end_date: Option<String>,
    /// pipeline status
    pub status: PipelineStatus,
    /// pipeline job id
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: String,
    /// pipeline agent id
    #[serde(rename(deserialize = "agentId", deserialize = "agent_id"))]
    pub agent_id: Option<String>,
}

/// A struct representing the registration of a pipeline.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterPipeline {
    /// pipeline job id
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: String,
    /// pipeline branch
    pub branch: Option<String>,
}

impl RegisterPipeline {
    /// constructor
    #[must_use]
    pub fn new(job_id: &str) -> Self {
        Self {
            job_id: job_id.to_string(),
            branch: Some("master".to_string()),
        }
    }
}

impl From<&RegisterPipeline> for Pipeline {
    fn from(value: &RegisterPipeline) -> Self {
        Self {
            id: Self::generate_id(),
            number: 0,
            branch: value.clone().branch.unwrap_or_default(),
            register_date: String::new(),
            start_date: None,
            end_date: None,
            status: PipelineStatus::Defined,
            job_id: value.clone().job_id,
            agent_id: None,
        }
    }
}

impl RustyDomainItem for Pipeline {}

/// A struct representing a paged result Pipelines.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedPipelines {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<Pipeline>,
}
