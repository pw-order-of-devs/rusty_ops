use mockito::{Mock, ServerGuard};

use domain::pipelines::{Pipeline, RegisterPipeline};
use rusty_agent::resolver::execution;

use crate::rusty_agent::api::mockito_start_server;

#[tokio::test]
async fn execute_pipeline_test() {
    let pipeline = Pipeline::from(&RegisterPipeline {
        job_id: "dummy".to_string(),
    });
    let mut server = mockito_start_server().await;
    let _ = mock_server_request(&mut server).await;
    let result = execution::execute_pipeline(pipeline, "uuid").await;
    assert!(result.is_ok());
}

async fn mock_server_request(server: &mut ServerGuard) -> Mock {
    server
        .mock("POST", "/graphql")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{"data": {{
                "jobs": {{
                    "getById": {{
                        "projectId": "dummy",
                        "template": "{}"
                    }}
                }},
                "projects": {{
                    "getById": {{
                        "url": "https://github.com/pw-order-of-devs/rusty_ops"
                    }}
                }}
            }} }}"#,
            template_yaml()
        ))
        .create()
}

fn template_yaml() -> String {
    let yaml = r#"
    stages:
       test:
          script:
            - echo "hello"
    "#;

    base64_url::encode(&yaml)
}
