use std::time::Duration;

use commons::env::var_or_default;
use domain::auth::credentials::Credential;
use domain::pipelines::PipelineStatus;
use persist::db_client::DbClient;

use crate::services::{agents, pipelines};

pub async fn schedule(db: &DbClient) {
    let timer = var_or_default("SCHEDULER_PIPELINES_CLEANUP", 60);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        task.tick().await;
        log::trace!("running `pipelines::cleanup` scheduled task");
        if let Ok(pipes) = pipelines::get_all(db, &Credential::System, &None, &None).await {
            for pipe in pipes {
                if [PipelineStatus::Assigned, PipelineStatus::InProgress].contains(&pipe.status) {
                    let agent =
                        agents::get_by_id(db, &Credential::System, &pipe.agent_id.unwrap()).await;
                    if agent.is_ok() && agent.unwrap().is_none() {
                        let _ = pipelines::reset(db, &Credential::System, &pipe.id).await;
                        log::debug!("pipeline `{}` reassigned.", &pipe.id);
                    }
                }
            }
        }
    }
}
