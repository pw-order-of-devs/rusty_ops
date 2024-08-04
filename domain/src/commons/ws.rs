use serde::{Deserialize, Serialize};

/// `WebSockets` extra payload
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExtraWSData {
    /// job id for pipeline subscriptions
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: Option<String>,
    /// pipeline id for pipeline logs subscriptions
    #[serde(rename(deserialize = "pipelineId", deserialize = "pipeline_id"))]
    pub pipeline_id: Option<String>,
}
