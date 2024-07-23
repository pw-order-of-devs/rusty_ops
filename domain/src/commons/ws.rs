use serde::{Deserialize, Serialize};

/// `WebSockets` extra payload
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExtraWSData {
    /// job id for pipeline subscriptions
    #[serde(rename(deserialize = "jobId", deserialize = "job_id"))]
    pub job_id: Option<String>,
}
