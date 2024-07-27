use std::time::Duration;

use commons::env::var_or_default;

use crate::api::pipelines;

// schedule a task every x minutes to fetch unassigned pipelines
pub async fn schedule(uuid: &str) {
    let timer = var_or_default("SCHEDULER_GET_UNASSIGNED", 300);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        log::trace!("fetching unassigned pipelines");
        task.tick().await;
        if let Ok(pipe) = pipelines::get_unassigned_pipeline().await {
            let _ = pipelines::assign_pipeline(&pipe.id, uuid).await;
        }
    }
}
