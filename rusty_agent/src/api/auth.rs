use commons::errors::RustyError;

use crate::api::client::{reqwest_post_basic, reqwest_post_bearer};

/// Authenticates the user and returns a login token.
///
/// # Returns
///
/// - `Ok(token)`: If the authentication is successful, returns the login token as a `String`.
/// - `Err(error)`: If there is an error during authentication, returns a `RustyError`.
#[allow(clippy::future_not_send)]
pub async fn authenticate() -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": "query { auth { login } }",
        "variables": {}
    });

    let data = reqwest_post_basic(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["auth"]["login"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

#[allow(clippy::future_not_send)]
pub async fn renew_token() -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": "query { auth { renew } }",
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["auth"]["renew"]
        .as_str()
        .unwrap_or("")
        .to_string())
}
