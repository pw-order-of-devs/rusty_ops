use commons::errors::RustyError;

use crate::api::client::reqwest_post;

/// Function to retrieve a projects from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_pipeline_repository(id: String) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            projects {{
                getById(id: "{}") {{
                    url
                }}
            }}
        }}"#, id),
        "variables": {}
    });

    let data = reqwest_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    json_data["data"]["projects"]["getById"]["url"]
        .as_str()
        .map_or_else(
            || Err(RustyError::RequestError("No results".to_string())),
            |value| Ok(value.to_string()),
        )
}
