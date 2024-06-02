use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use domain::RustyDomainItem;

#[test]
fn from_register_pipeline_test() {
    let job_id = uuid::Uuid::new_v4().to_string();
    let input = RegisterPipeline::new(&job_id);
    let pipeline = Pipeline::from(&input);
    assert_eq!(36, pipeline.get_id().len());
    assert_eq!(job_id, pipeline.job_id);
    assert_eq!(PipelineStatus::Defined, pipeline.status);
    assert_eq!(None, pipeline.agent_id);
    assert_eq!(None, pipeline.start_date);
    assert_eq!(None, pipeline.end_date);
}
