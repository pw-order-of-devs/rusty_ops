use commons::errors::RustyError;

use crate::api::client::reqwest_post_bearer;

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

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["agents"]["register"].to_string())
}

/// Function to unregister an agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn unregister(uuid: &str) -> Result<u64, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            agents {{
                deleteById(id: "{}")
            }}
        }}"#, uuid),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["agents"]["deleteById"]
        .as_u64()
        .unwrap_or(0))
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

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["agents"]["healthcheck"].to_string())
}
