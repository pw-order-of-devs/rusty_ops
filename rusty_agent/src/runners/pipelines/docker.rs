use crate::api::pipelines::update_stage;
use crate::runners::pipelines::shared;
use bollard::container::Config;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use commons::errors::RustyError;
use commons::errors::RustyError::DockerError;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::templates::pipeline::{PipelineTemplate, Script, Stage};
use futures_util::future::try_join_all;
use futures_util::{StreamExt, TryStreamExt};
use messaging::mq_client::MqClient;
use tokio::spawn;
use tokio::time::Instant;

pub async fn execute_docker(
    messaging: &MqClient,
    pipeline: &Pipeline,
    template: &PipelineTemplate,
    repo_url: &str,
    branch: &str,
    docker_image: &str,
    agent_uuid: &str,
) -> Result<(), RustyError> {
    let docker = Docker::connect_with_local_defaults()?;
    if clone_repository(&docker, messaging, repo_url, branch, &pipeline.id)
        .await
        .is_err()
    {
        shared::cleanup(
            messaging,
            agent_uuid,
            &pipeline.id,
            "rusty-before",
            PipelineStatus::Failure,
        )
        .await;
    };

    execute_stage(
        &docker,
        messaging,
        docker_image,
        &pipeline.id,
        agent_uuid,
        &template.before,
        &prepare_env(template, &None),
        "rusty-before",
    )
    .await?;

    let stages_tree = template.dependency_tree();
    for branch in stages_tree {
        let mut tasks = Vec::new();

        for leaf in branch {
            let docker = docker.clone();
            let docker_image = docker_image.to_string();
            let uuid = agent_uuid.to_string();
            let template = template.clone();
            let pipeline = pipeline.clone();
            let messaging = messaging.clone();

            let task = spawn(async move {
                let start = Instant::now();
                let (name, stage) = template.stages.iter().find(|(n, _)| leaf == **n).unwrap();
                let docker_image = if let Some(image) = stage.clone().image {
                    image
                } else {
                    docker_image
                };
                log::debug!("running stage: {name}");

                if let Err(err) = execute_stage(
                    &docker,
                    &messaging,
                    &docker_image,
                    &pipeline.id,
                    &uuid,
                    &Some(Script::new(&stage.script)),
                    &prepare_env(&template, &Some(stage.clone())),
                    "rusty-before",
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
        &docker,
        messaging,
        docker_image,
        &pipeline.id,
        agent_uuid,
        &template.after,
        &prepare_env(template, &None),
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

    Ok(())
}

async fn clone_repository(
    docker: &Docker,
    messaging: &MqClient,
    repo_url: &str,
    branch: &str,
    pipeline_id: &str,
) -> Result<(), RustyError> {
    create_image(docker, "alpine:3.20").await?;
    let container_id =
        create_container(docker, "alpine:3.20", "/tmp/rusty:/tmp/rusty".to_string()).await?;
    start_container(docker, &container_id).await?;
    execute_command(
        docker,
        messaging,
        shared::WORKING_DIR,
        &container_id,
        &split_command("apk add git"),
        &[],
        pipeline_id,
        "rusty-before",
    )
    .await?;
    let clone_command = format!(
        "git clone {repo_url} -b {branch} {}/{pipeline_id}",
        shared::WORKING_DIR
    );
    execute_command(
        docker,
        messaging,
        shared::WORKING_DIR,
        &container_id,
        &split_command(&clone_command),
        &[],
        pipeline_id,
        "rusty-before",
    )
    .await?;
    stop_container(docker, &container_id).await?;
    remove_container(docker, &container_id).await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn execute_stage(
    docker: &Docker,
    messaging: &MqClient,
    docker_image: &str,
    pipeline_id: &str,
    uuid: &str,
    script: &Option<Script>,
    env: &[String],
    stage_name: &str,
) -> Result<(), RustyError> {
    let _ = update_stage(pipeline_id, uuid, stage_name, PipelineStatus::InProgress).await;

    if let Some(script) = &script {
        create_image(docker, docker_image).await?;
        let working_dir = format!("{}/{pipeline_id}", shared::WORKING_DIR);
        let container_id =
            create_container(docker, docker_image, format!("{working_dir}:{working_dir}")).await?;
        start_container(docker, &container_id).await?;

        for command in &script.script {
            if let Err(err) = execute_command(
                docker,
                messaging,
                // &working_dir,
                "/",
                &container_id,
                &split_command(command),
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

        stop_container(docker, &container_id).await?;
        remove_container(docker, &container_id).await?;
    }

    let _ = update_stage(pipeline_id, uuid, stage_name, PipelineStatus::Success).await;
    Ok(())
}

async fn create_image(docker: &Docker, docker_image: &str) -> Result<(), RustyError> {
    let _ = docker
        .create_image(
            Some(CreateImageOptions {
                from_image: docker_image,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;
    log::debug!("Image created: {docker_image}");

    Ok(())
}

async fn create_container(
    docker: &Docker,
    docker_image: &str,
    volume_path: String,
) -> Result<String, RustyError> {
    let config = Config {
        image: Some(docker_image),
        tty: Some(true),
        host_config: Some(HostConfig {
            binds: Some(vec![volume_path]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let container = docker.create_container::<&str, &str>(None, config).await?;
    log::debug!("Container created: {:?}", container.id);
    Ok(container.id)
}

async fn start_container(docker: &Docker, container_id: &str) -> Result<(), RustyError> {
    docker.start_container::<String>(container_id, None).await?;
    log::debug!("Container started: {:?}", container_id);

    Ok(())
}

async fn stop_container(docker: &Docker, container_id: &str) -> Result<(), RustyError> {
    let _ = docker.stop_container(container_id, None).await;
    log::debug!("Container stopped: {:?}", container_id);

    Ok(())
}

async fn remove_container(docker: &Docker, container_id: &str) -> Result<(), RustyError> {
    let _ = docker.remove_container(container_id, None).await;
    log::debug!("Container removed: {:?}", container_id);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn execute_command(
    docker: &Docker,
    messaging: &MqClient,
    dir: &str,
    container_id: &str,
    command: &[String],
    env: &[String],
    pipeline_id: &str,
    stage: &str,
) -> Result<(), RustyError> {
    let exec_id = docker
        .create_exec(
            container_id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                env: Some(env.to_vec()),
                working_dir: Some(dir.to_string()),
                cmd: Some(command.to_vec()),
                ..Default::default()
            },
        )
        .await?
        .id;

    if let StartExecResults::Attached { mut output, .. } = docker.start_exec(&exec_id, None).await?
    {
        while let Some(Ok(msg)) = output.next().await {
            let line = msg.to_string().trim_end_matches('\n').to_string();
            let _ = shared::print_line(messaging, pipeline_id, stage, &line).await;
        }
    } else {
        unreachable!();
    }

    if let Some(exit_code) = docker.inspect_exec(&exec_id).await?.exit_code {
        if exit_code != 0 {
            return Err(DockerError(format!(
                "command terminated with {exit_code} exit code"
            )));
        }
    }

    Ok(())
}

fn split_command(command: &str) -> Vec<String> {
    shlex::split(command).unwrap_or_else(|| vec![command.to_string()])
}

fn prepare_env(template: &PipelineTemplate, stage: &Option<Stage>) -> Vec<String> {
    shared::prepare_env(template, stage)
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect()
}
