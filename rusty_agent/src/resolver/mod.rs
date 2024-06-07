use std::time::Duration;

use commons::env::var_or_default;
use domain::auth::credentials::get_token_claim_u64;

use crate::api::{agents, auth, pipelines, JWT_TOKEN};
use crate::resolver::execution::execute_pipeline;
use crate::resolver::subscription::pipeline_created_subscription;

pub mod assignment;
pub mod execution;
pub mod subscription;

/// initialization of schedulers handling pipelines
pub fn init(uuid: &str) {
    if var_or_default("SUBSCRIPTION_ENABLED", true) {
        let uuid_subscription = uuid.to_string();
        tokio::spawn(async move {
            created_pipelines_subscribe(&uuid_subscription).await;
        });
    }

    let uuid_schedule_get_unassigned = uuid.to_string();
    tokio::spawn(async move {
        fetch_unassigned_pipeline_schedule(&uuid_schedule_get_unassigned).await;
    });

    let uuid_schedule_get_assigned = uuid.to_string();
    tokio::spawn(async move {
        fetch_and_run_assigned_pipeline_schedule(&uuid_schedule_get_assigned).await;
    });

    let uuid_healthcheck = uuid.to_string();
    tokio::spawn(async move {
        healthcheck_schedule(&uuid_healthcheck).await;
    });

    tokio::spawn(async move {
        // by default - 60 seconds
        renew_token_schedule(60).await;
    });
}

pub async fn created_pipelines_subscribe(uuid: &str) {
    loop {
        log::trace!("connecting to subscription for newly created pipelines");
        match pipeline_created_subscription(uuid).await {
            Ok(()) => log::warn!("Connection was closed. Attempting to reconnect..."),
            Err(err) => log::warn!("An error occurred: {err}. Attempting to reconnect..."),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

// schedule a task every x minutes to fetch unassigned pipelines
pub async fn fetch_unassigned_pipeline_schedule(uuid: &str) {
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

// schedule a task every x minutes to fetch assigned pipeline and run it
pub async fn fetch_and_run_assigned_pipeline_schedule(uuid: &str) {
    let timer = var_or_default("SCHEDULER_GET_ASSIGNED", 300);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        log::trace!("fetching assigned pipelines");
        task.tick().await;
        if let Ok(pipe) = pipelines::get_last_assigned_pipeline(uuid).await {
            if pipelines::set_running(&pipe.clone().id, uuid).await.is_ok() {
                let _ = execute_pipeline(pipe, uuid).await;
            }
        }
    }
}

// schedule a task every x minutes to call the server with healthcheck
pub async fn healthcheck_schedule(uuid: &str) {
    let timer = var_or_default("SCHEDULER_HEALTHCHECK", 180);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    loop {
        log::trace!("calling healthcheck");
        task.tick().await;
        let _ = agents::healthcheck(uuid).await;
    }
}

// schedule a task every x minutes to renew jwt token before it terminates
pub async fn renew_token_schedule(wait_time: u64) {
    fn is_valid_jwt(token: &str) -> bool {
        token.split('.').count() == 3
    }

    fn calc_wait_time(token: &str) -> u64 {
        let now = chrono::Utc::now().timestamp();
        let now = if now.is_negative() { 0 } else { now as u64 };

        let expiry = get_token_claim_u64(token, "exp");
        let expiry = if expiry < now { now } else { expiry };

        let wait_time = ((expiry - now) as f64 * 0.9).round();
        wait_time.clamp(0., u64::MAX as f64) as u64
    }

    let mut wait_time = wait_time;

    loop {
        log::trace!("waiting for jwt token to be obtained");
        let token = JWT_TOKEN.lock().unwrap().clone();
        if is_valid_jwt(&token) {
            let wait = calc_wait_time(&token);
            wait_time = if wait == 0 { wait_time } else { wait };
            tokio::time::sleep(Duration::from_secs(wait_time)).await;
            break;
        }
        tokio::time::sleep(Duration::from_secs(wait_time)).await;
    }

    loop {
        log::trace!("attempting to renew jwt token");
        let token = auth::renew_token().await.unwrap_or_default();
        if is_valid_jwt(&token) {
            let wait = calc_wait_time(&token);
            wait_time = if wait == 0 { wait_time } else { wait };
            *JWT_TOKEN.lock().unwrap() = token;
            log::trace!("renewed jwt token");
        }
        tokio::time::sleep(Duration::from_secs(wait_time)).await;
    }
}
