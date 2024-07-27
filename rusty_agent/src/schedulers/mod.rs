pub mod healthcheck;
pub mod pipeline_created;
pub mod pipeline_fetch_assigned;
pub mod pipeline_fetch_unassigned;
pub mod renew_token;

use commons::env::var_or_default;

/// initialization of pipeline handling schedulers
pub fn init(uuid: &str) {
    if var_or_default("SUBSCRIPTION_ENABLED", true) {
        let uuid_subscription = uuid.to_string();
        tokio::spawn(async move {
            pipeline_created::subscribe(&uuid_subscription).await;
        });
    }

    let uuid_schedule_get_unassigned = uuid.to_string();
    tokio::spawn(async move {
        pipeline_fetch_unassigned::schedule(&uuid_schedule_get_unassigned).await;
    });

    let uuid_schedule_get_assigned = uuid.to_string();
    tokio::spawn(async move {
        pipeline_fetch_assigned::schedule(&uuid_schedule_get_assigned).await;
    });

    let uuid_healthcheck = uuid.to_string();
    tokio::spawn(async move {
        healthcheck::schedule(&uuid_healthcheck).await;
    });

    tokio::spawn(async move {
        // by default - 60 seconds
        renew_token::schedule(60).await;
    });
}
