use std::time::Duration;

use crate::api::pipelines::{get_last_assigned_pipeline, set_running};
use crate::resolver::execution::execute_pipeline;

use crate::resolver::subscription::pipeline_created_subscription;

mod execution;
mod subscription;

/// initialization of schedulers handling pipelines
pub fn init() {
    let uuid = uuid::Uuid::new_v4().to_string();
    log::debug!("Initialized with id: `{uuid}`");

    // listen for newly created pipelines
    let uuid_subscription = uuid.clone();
    tokio::spawn(async move {
        loop {
            match pipeline_created_subscription(&uuid_subscription).await {
                Ok(()) => log::warn!("Connection was closed. Attempting to reconnect..."),
                Err(err) => log::warn!("An error occurred: {err}. Attempting to reconnect..."),
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // schedule a task every x minutes to fetch assigned pipeline and run it
    let uuid_schedule_get_assigned = uuid;
    tokio::spawn(async move {
        let mut task = tokio::time::interval(Duration::from_secs(5));

        loop {
            task.tick().await;
            if let Ok(pipe) = get_last_assigned_pipeline(&uuid_schedule_get_assigned).await {
                if set_running(&pipe.clone().id, &uuid_schedule_get_assigned)
                    .await
                    .is_ok()
                {
                    let _ = execute_pipeline(pipe, &uuid_schedule_get_assigned).await;
                }
            }
        }
    });

    // schedule a task every x minutes to fetch unassigned pipelines
}
