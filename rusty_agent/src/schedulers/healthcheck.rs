use std::time::Duration;

use commons::env::var_or_default;

use crate::api::agents;

// schedule a task every x minutes to call the server with healthcheck
pub async fn schedule(uuid: &str) {
    let timer = var_or_default("SCHEDULER_HEALTHCHECK", 180);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        log::trace!("calling healthcheck");
        task.tick().await;
        let _ = agents::healthcheck(uuid).await;
    }
}
