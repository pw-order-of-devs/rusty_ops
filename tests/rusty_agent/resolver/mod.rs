use mockito::{Mock, ServerGuard};
use tokio::time::{timeout, Duration};

use rusty_agent::api::JWT_TOKEN;
use rusty_agent::resolver;

use crate::utils::mockito_start_server;

mod assignment;
mod execution;

#[tokio::test]
async fn resolver_init_test() {
    let _ = resolver::init("uuid");
}

#[tokio::test]
async fn created_pipelines_subscribe_test() {
    let handle = tokio::spawn(resolver::created_pipelines_subscribe("ok"));
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn fetch_unassigned_pipeline_schedule_test() {
    let handle = tokio::spawn(resolver::fetch_unassigned_pipeline_schedule("ok"));
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn fetch_and_run_assigned_pipeline_schedule_test() {
    let handle = tokio::spawn(resolver::fetch_and_run_assigned_pipeline_schedule("ok"));
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn healthcheck_schedule_test() {
    let handle = tokio::spawn(resolver::healthcheck_schedule("ok"));
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn renew_token_schedule_invalid_token_test() {
    let handle = tokio::spawn(resolver::renew_token_schedule(60));
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn renew_token_schedule_valid_token_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server).await;

    *JWT_TOKEN.lock().unwrap() = "eyJhbGciOiJIUzUxMiJ9.eyJpc3MiOiJSdXN0eU9wcyIsInN1YiI6InVzZXIiLCJhdWQiOiJ1c2VyIiwiZXhwIjoxNjE3MDEwNDg4LCJuYmYiOjE2MTcwMTA0ODgsImlhdCI6MTYxNzAxMDQ4OCwianRpIjoiYTQyZDYyN2YtYTEwMC00OWViLTg0MDYtMWZkMWMzMmI2MDMxIn0.".to_string();
    let handle = tokio::spawn(resolver::renew_token_schedule(3));
    let result = timeout(Duration::from_secs(5), handle).await;
    assert!(result.is_err());
    mock.assert();
}

async fn mock_server_request(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(r#"{{"data": {{"auth": {{"renew": "eyJhbGciOiJIUzUxMiJ9.eyJpc3MiOiJSdXN0eU9wcyIsInN1YiI6InVzZXIiLCJhdWQiOiJ1c2VyIiwiZXhwIjoxNjE3MDEwNDg4LCJuYmYiOjE2MTcwMTA0ODgsImlhdCI6MTYxNzAxMDQ4OCwianRpIjoiYTQyZDYyN2YtYTEwMC00OWViLTg0MDYtMWZkMWMzMmI2MDMxIn0."}}}}}}"#))
        .create()
}
