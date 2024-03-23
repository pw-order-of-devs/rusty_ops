use commons::errors::RustyError;

use crate::api::client::reqwest_post;

/// Function to register an agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn register(uuid: &str) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            agents {{
                register(agent: {{
                    id: "{}"
                }})
            }}
        }}"#, uuid),
        "variables": {}
    });

    let data = reqwest_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["agents"]["register"].to_string())
}

/// Function to call a healthcheck for agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn healthcheck(uuid: &str) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            agents {{
                healthcheck(id: "{}")
            }}
        }}"#, uuid),
        "variables": {}
    });

    let data = reqwest_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["agents"]["healthcheck"].to_string())
}
