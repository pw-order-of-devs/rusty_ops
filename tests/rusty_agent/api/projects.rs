use mockito::{Mock, ServerGuard};

use crate::utils::mockito_start_server;

#[tokio::test]
async fn get_pipeline_repository_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server).await;
    let result = rusty_agent::api::projects::get_pipeline_repository("id").await;
    assert!(result.is_ok());
    mock.assert();
}

async fn mock_server_request(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{"projects": {{"getById": {{"url": "http://dummy.ext"}} }} }} }}"#
        ))
        .create()
}
