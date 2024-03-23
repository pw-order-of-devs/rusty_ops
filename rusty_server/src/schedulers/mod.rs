use std::time::Duration;

use commons::env::var_or_default;
use persist::db_client::DbClient;

use crate::services::agents as agents_service;

/// initialization of schedulers
pub fn init(db: &DbClient) {
    let db = db.clone();
    tokio::spawn(async move {
        let timer = var_or_default("SCHEDULER_AGENTS_TTL", 60);
        let mut task = tokio::time::interval(Duration::from_secs(timer));

        loop {
            task.tick().await;
            log::trace!("running `agents::expirt` scheduled task");
            if let Ok(agents) = agents_service::get_all(&db, None, None).await {
                for agent in agents {
                    if agent.expiry < chrono::Utc::now().timestamp() {
                        log::debug!("agent `{}` expired.", &agent.id);
                        let _ = agents_service::delete_by_id(&db, &agent.id).await;
                    }
                }
            }
        }
    });
}
