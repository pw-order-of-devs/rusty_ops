use std::time::Duration;

use commons::env::var_or_default;
use domain::auth::credentials::Credential;
use domain::pipelines::PipelineStatus;
use persist::db_client::DbClient;

use crate::services::agents as agents_service;
use crate::services::pipelines as pipelines_service;

/// initialization of schedulers
pub fn init(db: &DbClient) {
    // scheduler for agent expiry - remove agent reference after expiration
    let db_agents = db.clone();
    tokio::spawn(async move {
        scheduler_agent_ttl(&db_agents).await;
    });

    // scheduler for pipelines with unknown agent - clean up status if assigned to nonexistent agent
    let db_pipelines = db.clone();
    tokio::spawn(async move {
        scheduler_pipelines_cleanup(&db_pipelines).await;
    });
}

pub async fn scheduler_agent_ttl(db: &DbClient) {
    let timer = var_or_default("SCHEDULER_AGENTS_TTL", 60);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        task.tick().await;
        log::trace!("running `agents::expire` scheduled task");
        if let Ok(agents) = agents_service::get_all(db, &None, &None).await {
            for agent in agents {
                if agent.expiry < chrono::Utc::now().timestamp() {
                    log::debug!("agent `{}` expired.", &agent.id);
                    let _ = agents_service::delete_by_id(db, &agent.id).await;
                }
            }
        }
    }
}

pub async fn scheduler_pipelines_cleanup(db: &DbClient) {
    let timer = var_or_default("SCHEDULER_PIPELINES_CLEANUP", 60);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        task.tick().await;
        log::trace!("running `pipelines::cleanup` scheduled task");
        if let Ok(pipes) = pipelines_service::get_all(db, &Credential::System, &None, &None).await {
            for pipe in pipes {
                if [PipelineStatus::Assigned, PipelineStatus::InProgress].contains(&pipe.status) {
                    let agent = agents_service::get_by_id(db, &pipe.agent_id.unwrap()).await;
                    if agent.is_ok() && agent.unwrap().is_none() {
                        let _ = pipelines_service::reset(db, &Credential::System, &pipe.id).await;
                        log::debug!("pipeline `{}` reassigned.", &pipe.id);
                    }
                }
            }
        }
    }
}
