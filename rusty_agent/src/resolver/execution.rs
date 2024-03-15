use std::io::Write;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};

use crate::api::jobs::get_pipeline_template;
use crate::api::pipelines::finalize;

pub(crate) async fn execute_pipeline(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);
    let template = get_pipeline_template(pipeline.job_id).await?;
    // if image: run in docker
    // get temp location
    // clone project
    for (name, stage) in template.stages {
        log::debug!("running stage: {name}");
        // if image: run in docker
        for command in stage.script {
            let _ = run_bash_command(&command);
        }
    }

    let _ = finalize(&pipeline.id, uuid, PipelineStatus::Success).await;
    log::debug!("done: running pipeline {}", pipeline.id);
    Ok(())
}

pub fn run_bash_command(command: &str) -> std::io::Result<()> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if !output.status.success() {
        eprintln!("Command executed with error: {}", output.status);
    }

    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;

    Ok(())
}
