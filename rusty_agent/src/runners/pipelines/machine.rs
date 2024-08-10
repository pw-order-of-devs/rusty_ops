use std::collections::HashMap;

use futures_util::future::try_join_all;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::process::Command;
use tokio::spawn;
use tokio::time::Instant;

use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::templates::pipeline::{PipelineTemplate, Script};
use messaging::mq_client::MqClient;

use crate::api::pipelines::update_stage;
use crate::runners::pipelines::shared;

pub async fn execute_machine(
    messaging: &MqClient,
    pipeline: &Pipeline,
    template: &PipelineTemplate,
    repo_url: &str,
    branch: &str,
    agent_uuid: &str,
) -> Result<(), RustyError> {
    std::fs::create_dir_all(shared::WORKING_DIR)?;
    clone_repository(agent_uuid, &pipeline.id, repo_url, branch, messaging).await?;

    execute_stage(
        messaging,
        &pipeline.id,
        agent_uuid,
        &template.before,
        &shared::prepare_env(template, &None),
        "rusty-before",
    )
    .await?;

    let stages_tree = template.dependency_tree();
    for branch in stages_tree {
        let mut tasks = Vec::new();

        for leaf in branch {
            let uuid = agent_uuid.to_string();
            let template = template.clone();
            let pipeline = pipeline.clone();
            let messaging = messaging.clone();

            let task = spawn(async move {
                let start = Instant::now();
                let (name, stage) = template.stages.iter().find(|(n, _)| leaf == **n).unwrap();
                log::debug!("running stage: {name}");
                if let Err(err) = execute_stage(
                    &messaging,
                    &pipeline.id,
                    &uuid,
                    &Some(Script::new(&stage.script)),
                    &shared::prepare_env(&template, &Some(stage.clone())),
                    name,
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
        messaging,
        &pipeline.id,
        agent_uuid,
        &template.after,
        &shared::prepare_env(template, &None),
        "rusty-after",
    )
    .await?;

    shared::cleanup(
        messaging,
        agent_uuid,
        &pipeline.id,
        "rusty-after",
        PipelineStatus::Success,
    )
    .await;
    log::debug!("done: running pipeline {}", pipeline.id);
    Ok(())
}

async fn clone_repository(
    uuid: &str,
    pipeline_id: &str,
    repo_url: &str,
    branch: &str,
    messaging: &MqClient,
) -> Result<(), RustyError> {
    log::debug!("cloning repository: {repo_url} -b {branch}");
    if let Err(err) = run_bash_command(
        messaging,
        shared::WORKING_DIR,
        &format!("git clone {repo_url} -b {branch} {pipeline_id}"),
        &HashMap::new(),
        pipeline_id,
        "rusty-before",
    )
    .await
    {
        log::error!("Error in pipeline {}: {}", &pipeline_id, err);
        shared::cleanup(
            messaging,
            uuid,
            pipeline_id,
            "rusty-before",
            PipelineStatus::Failure,
        )
        .await;
        Err(err)
    } else {
        Ok(())
    }
}

async fn execute_stage(
    messaging: &MqClient,
    pipeline_id: &str,
    uuid: &str,
    script: &Option<Script>,
    env: &HashMap<String, String>,
    stage_name: &str,
) -> Result<(), RustyError> {
    let _ = update_stage(pipeline_id, uuid, stage_name, PipelineStatus::InProgress).await;

    if let Some(script) = &script {
        for command in &script.script {
            if let Err(err) = run_bash_command(
                messaging,
                &format!("{}/{pipeline_id}", shared::WORKING_DIR),
                command,
                env,
                pipeline_id,
                stage_name,
            )
            .await
            {
                log::error!("Error in pipeline {}: {}", pipeline_id, err);
                shared::cleanup(
                    messaging,
                    uuid,
                    pipeline_id,
                    stage_name,
                    PipelineStatus::Failure,
                )
                .await;
                return Err(RustyError::IoError(format!(
                    "`{stage_name}` stage failed for pipeline `{pipeline_id}`"
                )));
            }
        }
    }

    let _ = update_stage(pipeline_id, uuid, stage_name, PipelineStatus::Success).await;
    Ok(())
}

async fn run_bash_command(
    messaging: &MqClient,
    dir: &str,
    command: &str,
    env: &HashMap<String, String>,
    pipeline_id: &str,
    stage: &str,
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
    let mq_out = messaging.clone();
    let id_out = pipeline_id.to_string();
    let stage_out = stage.to_string();
    let stdout_handle = spawn(async move {
        print_line(stdout, &mq_out, &id_out, &stage_out).await;
    });

    let stderr = process.stderr.take().unwrap();
    let mq_err = messaging.clone();
    let id_err = pipeline_id.to_string();
    let stage_err = stage.to_string();
    let stderr_handle = spawn(async move {
        print_line(stderr, &mq_err, &id_err, &stage_err).await;
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

async fn print_line(
    writer: impl AsyncRead + Unpin + Send,
    messaging: &MqClient,
    pipeline_id: &str,
    stage: &str,
) {
    let reader = BufReader::new(writer);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let _ = shared::print_line(messaging, pipeline_id, stage, &line).await;
    }
}
