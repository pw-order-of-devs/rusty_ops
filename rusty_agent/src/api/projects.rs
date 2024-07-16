use commons::errors::RustyError;

use crate::api::client::reqwest_post_bearer;

/// Function to retrieve a projects from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_pipeline_project(id: &str) -> Result<(String, String), RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            projects {{
                getById(id: "{}") {{
                    mainBranch
                    url
                }}
            }}
        }}"#, id),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    if let Some(project) = json_data["data"]["projects"]["getById"].as_object() {
        let branch = project["mainBranch"].as_str().unwrap_or_default();
        let url = project["url"].as_str().unwrap_or_default();
        Ok((branch.to_string(), url.to_string()))
    } else {
        Err(RustyError::RequestError("No results".to_string()))
    }
}
