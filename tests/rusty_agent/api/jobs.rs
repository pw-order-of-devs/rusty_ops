use mockito::{Mock, ServerGuard};

use crate::rusty_agent::api::mockito_start_server;

#[tokio::test]
async fn get_pipeline_template_test() {
    let mut server = mockito_start_server().await;
    let mock = mock_server_request(&mut server).await;
    let result = rusty_agent::api::jobs::get_pipeline_template("id").await;
    assert!(result.is_ok());
    mock.assert();
}

async fn mock_server_request(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{"jobs": {{"getById": {{"template": "c3RhZ2VzOgogICB0ZXN0OgogICAgICBzY3JpcHQ6CiAgICAgICAgLSBlY2hvICJoZWxsbyI"}} }} }} }}"#
        ))
        .create()
}
