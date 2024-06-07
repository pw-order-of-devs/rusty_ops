use mockito::{Mock, ServerGuard};

use crate::rusty_agent::api::mockito_start_server;

#[tokio::test]
async fn register_agent_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server, "register").await;
    let result = rusty_agent::api::agents::register("ok").await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn unregister_agent_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server, "unregister").await;
    let result = rusty_agent::api::agents::unregister("ok").await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn healthcheck_agent_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server, "healthcheck").await;
    let result = rusty_agent::api::agents::healthcheck("ok").await;
    assert!(result.is_ok());
    mock.assert();
}

async fn mock_server_request(server: &mut ServerGuard, request: &str) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{"agents": {{"{request}": "ok" }}}}}}"#
        ))
        .create()
}
