use std::collections::HashMap;
use std::time::Instant;

use futures_util::future::try_join_all;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::process::Command;
use tokio::spawn;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::templates::pipeline::{PipelineTemplate, Script, Stage};

use crate::api::jobs::get_pipeline_template;
use crate::api::pipelines::finalize;
use crate::api::projects::get_pipeline_project;

pub async fn execute_pipeline(pipeline: Pipeline, uuid: &str) -> Result<(), RustyError> {
    log::debug!("running pipeline {}", pipeline.id);
    let (project_id, template) = get_pipeline_template(&pipeline.job_id).await?;
    let (default_branch, repo_url) = get_pipeline_project(&project_id).await?;
    let branch = if pipeline.branch.is_empty() {
        default_branch
    } else {
        pipeline.branch.clone()
    };
    // if image: run in docker

    let working_directory = format!("/tmp/rusty/{}", uuid::Uuid::new_v4());
    std::fs::create_dir_all(&working_directory)?;
    clone_repository(&working_directory, uuid, &pipeline.id, &repo_url, &branch).await?;
    let project_directory = format!("{working_directory}/{}", &pipeline.id);

    execute_stage(
        &project_directory,
        &working_directory,
        &pipeline.id,
        uuid,
        &template.before,
        &prepare_env(&template, &None),
    )
    .await?;

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
                if let Err(err) = execute_stage(
                    &project_directory,
                    &working_directory,
                    &pipeline.id,
                    &uuid,
                    &Some(Script::new(&stage.script)),
                    &prepare_env(&template, &Some(stage.clone())),
                )
                .await
                {
                    log::error!("Error in pipeline {}: {}", &pipeline.id, err);
                    return Err(());
                }

                let duration = start.elapsed().as_millis();
                log::debug!("done: running stage: {name} in {duration} ms");
                Ok(())
            });
            tasks.push(task);
        }

        try_join_all(tasks).await?;
    }

    execute_stage(
        &project_directory,
        &working_directory,
        &pipeline.id,
        uuid,
        &template.after,
        &prepare_env(&template, &None),
    )
    .await?;

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

async fn execute_stage(
    project_directory: &str,
    working_directory: &str,
    pipeline_id: &str,
    uuid: &str,
    script: &Option<Script>,
    env: &HashMap<String, String>,
) -> Result<(), RustyError> {
    if let Some(script) = &script {
        for command in &script.script {
            if let Err(err) = run_bash_command(project_directory, command, env).await {
                log::error!("Error in pipeline {}: {}", pipeline_id, err);
                cleanup(
                    working_directory,
                    pipeline_id,
                    uuid,
                    PipelineStatus::Failure,
                )
                .await;
                return Err(RustyError::IoError(format!(
                    "`before` stage failed for pipeline `{}`",
                    pipeline_id
                )));
            }
        }
    }

    Ok(())
}

fn prepare_env(template: &PipelineTemplate, stage: &Option<Stage>) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    if let Some(env) = template.clone().env {
        for (k, v) in env {
            envs.insert(k, v);
        }
    }
    if let Some(stage) = stage {
        if let Some(env) = stage.clone().env {
            for (k, v) in env {
                envs.insert(k, v);
            }
        }
    }
    envs
}

async fn run_bash_command(
    dir: &str,
    command: &str,
    env: &HashMap<String, String>,
) -> Result<(), RustyError> {
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
        Err(RustyError::IoError(format!("Command error: {status}")))
    } else {
        Ok(())
    }
}

async fn clone_repository(
    dir: &str,
    uuid: &str,
    pipeline_id: &str,
    repo_url: &str,
    branch: &str,
) -> Result<(), RustyError> {
    log::debug!("cloning repository: {repo_url} -b {branch}");
    if let Err(err) = run_bash_command(
        dir,
        &format!("git clone {repo_url} -b {branch} {pipeline_id}"),
        &HashMap::new(),
    )
    .await
    {
        log::error!("Error in pipeline {}: {}", &pipeline_id, err);
        cleanup(dir, pipeline_id, uuid, PipelineStatus::Failure).await;
        Err(err)
    } else {
        Ok(())
    }
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
