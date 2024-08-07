use commons::errors::RustyError;
use domain::templates::pipeline::PipelineTemplate;

use crate::api::client::reqwest_post_bearer;

/// Function to retrieve a job from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_pipeline_template(id: &str) -> Result<(String, PipelineTemplate), RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            jobs {{
                getById(id: "{}") {{
                    projectId
                    template
                }}
            }}
        }}"#, id.to_string()),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    if let Some(job) = json_data["data"]["jobs"]["getById"].as_object() {
        let project_id = job["projectId"].as_str().unwrap_or_default();
        let template = job["template"].as_str().unwrap_or_default();
        Ok((
            project_id.to_string(),
            PipelineTemplate::from_yaml(template)?,
        ))
    } else {
        Err(RustyError::RequestError("No results".to_string()))
    }
}
