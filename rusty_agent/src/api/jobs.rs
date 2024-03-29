use commons::errors::RustyError;
use domain::templates::pipeline::PipelineTemplate;

use crate::api::client::reqwest_post;

/// Function to retrieve a job from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_pipeline_template(id: String) -> Result<(String, PipelineTemplate), RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            jobs {{
                getById(id: "{}") {{
                    projectId
                    template
                }}
            }}
        }}"#, id),
        "variables": {}
    });

    let data = reqwest_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let project_id = json_data["data"]["jobs"]["getById"]["projectId"]
        .as_str()
        .unwrap_or("");
    json_data["data"]["jobs"]["getById"]["template"]
        .as_str()
        .map_or_else(
            || Err(RustyError::RequestError("No results".to_string())),
            |value| Ok((project_id.to_string(), PipelineTemplate::from_yaml(value)?)),
        )
}
