use std::time::Duration;

use commons::env::var_or_default;

use crate::api::pipelines;
use crate::runners;

// schedule a task every x minutes to fetch assigned pipeline and execute it
pub async fn schedule(uuid: &str) {
    let timer = var_or_default("SCHEDULER_GET_ASSIGNED", 300);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        log::trace!("fetching assigned pipelines");
        task.tick().await;
        if let Ok(pipe) = pipelines::get_last_assigned_pipeline(uuid).await {
            if pipelines::set_running(&pipe.clone().id, uuid).await.is_ok() {
                let _ = runners::pipelines::execute(pipe, uuid).await;
            }
        }
    }
}
