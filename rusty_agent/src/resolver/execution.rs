use std::fs::read_to_string;
use std::io::BufRead;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::templates::pipeline::PipelineTemplate;

use crate::api::jobs::get_pipeline_template;
use crate::api::pipelines::finalize;
use crate::api::projects::get_pipeline_repository;

pub(crate) async fn execute_pipeline(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);
    let (project_id, mut template) = get_pipeline_template(pipeline.job_id).await?;
    let repo_url = get_pipeline_repository(project_id).await?;
    // if image: run in docker

    let working_directory = format!("/tmp/rusty/{}", uuid::Uuid::new_v4());
    std::fs::create_dir_all(&working_directory)?;
    clone_repository(&working_directory, uuid, &pipeline.id, &repo_url).await?;
    let project_directory = format!("{working_directory}/source");
    if let Ok(temp) = fetch_template_from_files(&project_directory) {
        log::debug!("found template in project files.");
        template = temp;
    } else {
        log::debug!("no template in project files, using default one.");
    };

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

    if let Some(ref mut stderr) = process.stderr {
        let reader = std::io::BufReader::new(stderr);

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

async fn clone_repository(
    dir: &str,
    uuid: &str,
    pipeline_id: &str,
    repo_url: &str,
) -> Result<(), RustyError> {
    if let Err(err) = run_bash_command(dir, &format!("git clone {repo_url} source")) {
        log::error!("Error in pipeline {}: {}", &pipeline_id, err);
        cleanup(dir, pipeline_id, uuid, PipelineStatus::Failure).await;
        Err(RustyError::from(err))
    } else {
        Ok(())
    }
}

fn fetch_template_from_files(dir: &str) -> Result<PipelineTemplate, RustyError> {
    let file = read_to_string(format!("{dir}/rusty_ci.yaml"))?;
    let file = base64_url::encode(&file);
    PipelineTemplate::from_yaml(&file)
}

async fn cleanup(dir: &str, pipe_id: &str, uuid: &str, status: PipelineStatus) {
    let _ = std::fs::remove_dir(dir);
    let _ = finalize(pipe_id, uuid, status).await;
}
