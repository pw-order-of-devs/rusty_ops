use std::time::Duration;

use commons::env::var_or_default;
use domain::auth::credentials::Credential;
use persist::db_client::DbClient;

use crate::services::agents;

pub async fn schedule(db: &DbClient) {
    let timer = var_or_default("SCHEDULER_AGENTS_TTL", 60);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        task.tick().await;
        log::trace!("running `agents::expire` scheduled task");
        if let Ok(agents) = agents::get_all(db, &Credential::System, &None, &None).await {
            for agent in agents {
                if agent.expiry < chrono::Utc::now().timestamp() {
                    log::debug!("agent `{}` expired.", &agent.id);
                    let _ = agents::delete_by_id(db, &Credential::System, &agent.id).await;
                }
            }
        }
    }
}
