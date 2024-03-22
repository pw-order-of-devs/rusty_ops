use std::io::BufRead;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};

use crate::api::jobs::get_pipeline_template;
use crate::api::pipelines::finalize;
use crate::api::projects::get_pipeline_repository;

pub(crate) async fn execute_pipeline(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);
    let (project_id, template) = get_pipeline_template(pipeline.job_id).await?;
    let repo_url = get_pipeline_repository(project_id).await?;
    // if image: run in docker

    let working_directory = format!("/tmp/{}", uuid::Uuid::new_v4());
    let _ = std::fs::create_dir(&working_directory);
    if let Err(err) = run_bash_command(&working_directory, &format!("git clone {repo_url} source"))
    {
        log::error!("Error in pipeline {}: {}", &pipeline.id, err);
        cleanup(
            &working_directory,
            &pipeline.id,
            uuid,
            PipelineStatus::Failure,
        )
        .await;
        return Ok(());
    }

    let project_directory = format!("{working_directory}/source");
    for (name, stage) in template.stages {
        log::debug!("running stage: {name}");
        // if image: run in docker
        for command in stage.script {
            if let Err(err) = run_bash_command(&project_directory, &command) {
                log::error!("Error in pipeline {}: {}", &pipeline.id, err);
                cleanup(
                    &working_directory,
                    &pipeline.id,
                    uuid,
                    PipelineStatus::Failure,
                )
                .await;
                return Ok(());
            }
        }
        log::debug!("done: running stage: {name}");
    }

    cleanup(
        &working_directory,
        &pipeline.id,
        uuid,
        PipelineStatus::Success,
    )
    .await;
    log::debug!("done: running pipeline {}", pipeline.id);
    Ok(())
}

fn run_bash_command(dir: &str, command: &str) -> std::io::Result<()> {
    let mut process = std::process::Command::new("sh")
        .current_dir(dir)
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdout) = process.stdout {
        let reader = std::io::BufReader::new(stdout);

        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    let output = process.wait_with_output()?;

    if !output.status.success() {
        // handle pipeline error
        log::error!("Command executed with error: {}", output.status);
    }

    Ok(())
}

async fn cleanup(dir: &str, pipe_id: &str, uuid: &str, status: PipelineStatus) {
    let _ = std::fs::remove_dir(dir);
    let _ = finalize(pipe_id, uuid, status).await;
}
