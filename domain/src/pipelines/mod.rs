use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::RustyDomainItem;

/// An enum representing a pipeline status.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum PipelineStatus {
    /// Created pipeline, waiting for agent to be assigned to.
    #[serde(rename(
        deserialize = "DEFINED",
        deserialize = "Defined",
        deserialize = "defined"
    ))]
    Defined,
    /// Pipeline assigned to an agent, not yet started.
    #[serde(rename(
        deserialize = "ASSIGNED",
        deserialize = "Assigned",
        deserialize = "assigned"
    ))]
    Assigned,
    /// Currently running pipeline.
    #[serde(rename(
        deserialize = "INPROGRESS",
        deserialize = "IN_PROGRESS",
        deserialize = "InProgress",
        deserialize = "in_progress"
    ))]
    InProgress,
    /// Pipeline finished successfully.
    #[serde(rename(
        deserialize = "SUCCESS",
        deserialize = "Success",
        deserialize = "success"
    ))]
    Success,
    /// Pipeline finished with a failure.
    #[serde(rename(
        deserialize = "FAILURE",
        deserialize = "Failure",
        deserialize = "failure"
    ))]
    Failure,
    /// Pipeline finished in an unstable state.
    #[serde(rename(
        deserialize = "UNSTABLE",
        deserialize = "Unstable",
        deserialize = "unstable"
    ))]
    Unstable,
}

/// A struct representing a pipeline.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Pipeline {
    /// pipeline id
    pub id: String,
    /// pipeline order number
    pub number: u64,
    /// pipeline start_date
    #[serde(rename(deserialize = "startDate", deserialize = "start_date"))]
    pub start_date: String,
    /// pipeline status
    pub status: PipelineStatus,
    /// pipeline job id
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: String,
}

/// A struct representing the registration of a pipeline.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct RegisterPipeline {
    /// pipeline job id
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: String,
}

impl RegisterPipeline {
    /// constructor
    #[must_use]
    pub fn new(job_id: &str) -> Self {
        Self {
            job_id: job_id.to_string(),
        }
    }
}

impl From<&RegisterPipeline> for Pipeline {
    fn from(value: &RegisterPipeline) -> Self {
        Self {
            id: Self::generate_id(),
            number: 0,
            start_date: String::new(),
            status: PipelineStatus::Defined,
            job_id: value.clone().job_id,
        }
    }
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
