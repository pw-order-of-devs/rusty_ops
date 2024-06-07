use mockito::{Mock, ServerGuard};

use crate::rusty_agent::api::mockito_start_server;

#[tokio::test]
async fn authenticate_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server, "login").await;
    let result = rusty_agent::api::auth::authenticate().await;
    assert!(result.is_ok());
    mock.assert();
}

#[tokio::test]
async fn renew_token_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server, "renew").await;
    let result = rusty_agent::api::auth::renew_token().await;
    assert!(result.is_ok());
    mock.assert();
}

async fn mock_server_request(server: &mut ServerGuard, request: &str) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(r#"{{"data": {{"auth": {{"{request}": "ok"}}}}}}"#))
        .create()
}
