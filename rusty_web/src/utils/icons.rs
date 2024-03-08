use domain::pipelines::PipelineStatus;

/// get display icon for pipeline status
#[must_use]
pub fn get_pipeline_status_icon(status: &PipelineStatus) -> String {
    match status {
        PipelineStatus::Defined |
        PipelineStatus::Assigned => "circle-dot",
        PipelineStatus::InProgress => "circle-play",
        PipelineStatus::Success => "circle-check",
        PipelineStatus::Failure => "circle-xmark",
        PipelineStatus::Unstable => "circle-exclamation",
    }.to_string()
}
