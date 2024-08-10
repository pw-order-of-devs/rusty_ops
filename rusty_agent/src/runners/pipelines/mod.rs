use commons::errors::RustyError;
use domain::pipelines::Pipeline;

use crate::api::jobs::get_pipeline_template;
use crate::api::projects::get_pipeline_project;
use crate::messaging::get_messaging;
use crate::runners::pipelines::{docker::execute_docker, machine::execute_machine};

mod docker;
mod machine;
mod shared;

pub async fn execute(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);

    let messaging = get_messaging().await?.lock().await;
    let _ = messaging
        .create_queue(&format!("pipeline-logs-{}", pipeline.id))
        .await;

    let (project_id, template) = get_pipeline_template(&pipeline.job_id).await?;
    let (default_branch, repo_url) = get_pipeline_project(&project_id).await?;
    let branch = if pipeline.branch.is_empty() {
        default_branch
    } else {
        pipeline.branch.clone()
    };

    match template.image {
        Some(ref image) => {
            execute_docker(
                &messaging, &pipeline, &template, &repo_url, &branch, image, uuid,
            )
            .await
        }
        None => execute_machine(&messaging, &pipeline, &template, &repo_url, &branch, uuid).await,
    }
}
