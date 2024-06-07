use futures_util::future::try_join_all;
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::process::Command;
use tokio::spawn;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::templates::pipeline::{PipelineTemplate, Stage};

use crate::api::jobs::get_pipeline_template;
use crate::api::pipelines::finalize;
use crate::api::projects::get_pipeline_repository;

pub(crate) async fn execute_pipeline(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);
    let (project_id, mut template) = get_pipeline_template(&pipeline.job_id).await?;
    let repo_url = get_pipeline_repository(&project_id).await?;
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

    let stages_tree = template.dependency_tree();
    for branch in stages_tree {
        let mut tasks = Vec::new();

        for leaf in branch {
            let uuid = uuid.to_string();
            let template = template.clone();
            let project_directory = project_directory.to_string();
            let working_directory = working_directory.to_string();
            let pipeline = pipeline.clone();

            let task = spawn(async move {
                let start = Instant::now();
                let (name, stage) = template.stages.iter().find(|(n, _)| leaf == **n).unwrap();
                log::debug!("running stage: {name}");
                // if image: run in docker
                let env = prepare_env(&template, stage);
                for command in &stage.script {
                    if let Err(err) = run_bash_command(&project_directory, command, &env).await {
                        log::error!("Error in pipeline {}: {}", &pipeline.id, err);
                        cleanup(
                            &working_directory,
                            &pipeline.id,
                            &uuid,
                            PipelineStatus::Failure,
                        )
                        .await;
                        return Err(());
                    }
                }
                let duration = start.elapsed().as_millis();
                log::debug!("done: running stage: {name} in {duration} ms");
                Ok(())
            });
            tasks.push(task);
        }

        try_join_all(tasks).await?;
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

fn prepare_env(template: &PipelineTemplate, stage: &Stage) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    if let Some(env) = template.clone().env {
        for (k, v) in env {
            envs.insert(k, v);
        }
    }
    if let Some(env) = stage.clone().env {
        for (k, v) in env {
            envs.insert(k, v);
        }
    }
    envs
}

async fn run_bash_command(
    dir: &str,
    command: &str,
    env: &HashMap<String, String>,
) -> std::io::Result<()> {
    let mut process = Command::new("sh")
        .current_dir(dir)
        .arg("-c")
        .arg(command)
        .envs(env)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let stdout = process.stdout.take().unwrap();

    let stdout_handle = spawn(async move {
        print_line(stdout).await;
    });

    let stderr = process.stderr.take().unwrap();
    let stderr_handle = spawn(async move {
        print_line(stderr).await;
    });

    let status = process.wait().await?;
    stdout_handle.await.unwrap();
    stderr_handle.await.unwrap();

    if !status.success() {
        // react to pipeline errors
        log::error!("Command executed with error: {}", status);
    }

    Ok(())
}

async fn clone_repository(
    dir: &str,
    uuid: &str,
    pipeline_id: &str,
    repo_url: &str,
) -> Result<(), RustyError> {
    if let Err(err) = run_bash_command(
        dir,
        &format!("git clone {repo_url} source"),
        &HashMap::new(),
    )
    .await
    {
        log::error!("Error in pipeline {}: {}", &pipeline_id, err);
        cleanup(dir, pipeline_id, uuid, PipelineStatus::Failure).await;
        Err(RustyError::from(err))
    } else {
        Ok(())
    }
}

fn fetch_template_from_files(dir: &str) -> Result<PipelineTemplate, RustyError> {
    let file = std::fs::read_to_string(format!("{dir}/rusty_ci.yaml"))?;
    let file = base64_url::encode(&file);
    PipelineTemplate::from_yaml(&file)
}

async fn cleanup(dir: &str, pipe_id: &str, uuid: &str, status: PipelineStatus) {
    let _ = std::fs::remove_dir_all(dir);
    let _ = finalize(pipe_id, uuid, status).await;
}

async fn print_line(writer: impl AsyncRead + Unpin + Send) {
    let reader = BufReader::new(writer);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        eprintln!("{line}");
    }
}
