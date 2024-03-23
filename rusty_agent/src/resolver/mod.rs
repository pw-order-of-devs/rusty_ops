use commons::env::var_or_default;
use std::time::Duration;

use crate::api::agents;
use crate::api::pipelines;
use crate::resolver::execution::execute_pipeline;
use crate::resolver::subscription::pipeline_created_subscription;

mod assignment;
mod execution;
mod subscription;

/// initialization of schedulers handling pipelines
pub fn init(uuid: String) {
    if var_or_default("SUBSCRIPTION_ENABLED", true) {
        let uuid_subscription = uuid.clone();
        tokio::spawn(async move {
            loop {
                log::trace!("connecting to subscription for newly created pipelines");
                match pipeline_created_subscription(&uuid_subscription).await {
                    Ok(()) => log::warn!("Connection was closed. Attempting to reconnect..."),
                    Err(err) => log::warn!("An error occurred: {err}. Attempting to reconnect..."),
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    // schedule a task every x minutes to fetch unassigned pipelines
    let uuid_schedule_get_unassigned = uuid.clone();
    tokio::spawn(async move {
        let timer = var_or_default("SCHEDULER_GET_UNASSIGNED", 300);
        let mut task = tokio::time::interval(Duration::from_secs(timer));

        loop {
            log::trace!("fetching unassigned pipelines");
            task.tick().await;
            if let Ok(pipe) = pipelines::get_unassigned_pipeline().await {
                let _ = pipelines::assign_pipeline(&pipe.id, &uuid_schedule_get_unassigned).await;
            }
        }
    });

    // schedule a task every x minutes to fetch assigned pipeline and run it
    let uuid_schedule_get_assigned = uuid.clone();
    tokio::spawn(async move {
        let timer = var_or_default("SCHEDULER_GET_ASSIGNED", 300);
        let mut task = tokio::time::interval(Duration::from_secs(timer));

        loop {
            log::trace!("fetching assigned pipelines");
            task.tick().await;
            if let Ok(pipe) =
                pipelines::get_last_assigned_pipeline(&uuid_schedule_get_assigned).await
            {
                if pipelines::set_running(&pipe.clone().id, &uuid_schedule_get_assigned)
                    .await
                    .is_ok()
                {
                    let _ = execute_pipeline(pipe, &uuid_schedule_get_assigned).await;
                }
            }
        }
    });

    // schedule a task every x minutes to call the server with healthcheck
    let uuid_healthcheck = uuid;
    tokio::spawn(async move {
        let timer = var_or_default("SCHEDULER_HEALTHCHECK", 180);
        let mut task = tokio::time::interval(Duration::from_secs(timer));

        loop {
            log::trace!("calling healthcheck");
            task.tick().await;
            let _ = agents::healthcheck(&uuid_healthcheck).await;
        }
    });
}
