use domain::pipelines::PipelineStatus;
use mockito::{Mock, ServerGuard};

use crate::utils::mockito_start_server;

#[tokio::test]
async fn get_unassigned_pipeline_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request_get(&mut server).await;
    let result = rusty_agent::api::pipelines::get_unassigned_pipeline().await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn get_last_assigned_pipeline_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request_get(&mut server).await;
    let result = rusty_agent::api::pipelines::get_last_assigned_pipeline("ok").await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn assign_pipeline_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request_put(&mut server, "assign").await;
    let result = rusty_agent::api::pipelines::assign_pipeline("ok", "ok").await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn set_running_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request_put(&mut server, "setRunning").await;
    let result = rusty_agent::api::pipelines::set_running("ok", "ok").await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn finalize_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request_put(&mut server, "finalize").await;
    let result = rusty_agent::api::pipelines::finalize("ok", "ok", PipelineStatus::Success).await;
    assert!(result.is_ok());
    mock.assert();
}

async fn mock_server_request_get(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{"pipelines": {{ "get": {{"entries": [{{
                "id": "c2b83cae-2e66-465b-a66d-573f41e90905",
                "number": 0,
                "branch": "master",
                "registerDate": "2024-05-31T11:08:36.556775405+00:00",
                "status": "DEFINED",
                "stageStatus": {{}},
                "jobId": "da30423b-92bb-47f3-9710-28b106c3cb0c"
            }}] }} }} }} }}"#
        ))
        .create()
}

async fn mock_server_request_put(server: &mut ServerGuard, request: &str) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{"pipelines": {{ "{request}": "ok" }} }} }}"#
        ))
        .create()
}
