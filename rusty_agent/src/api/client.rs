use commons::env::var_or_default;
use commons::errors::RustyError;

/// HTTP POST request with basic authentication
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `reqwest::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn reqwest_post_basic(payload: &serde_json::Value) -> Result<String, RustyError> {
    let cred = crate::api::get_credential().unwrap_or_default();
    reqwest_post(payload, &format!("Basic {cred}")).await
}

/// HTTP POST request with bearer authentication
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `reqwest::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn reqwest_post_bearer(payload: &serde_json::Value) -> Result<String, RustyError> {
    let jwt_token = crate::api::JWT_TOKEN.lock().unwrap().clone();
    reqwest_post(payload, &format!("Bearer {jwt_token}")).await
}

#[allow(clippy::future_not_send)]
async fn reqwest_post(payload: &serde_json::Value, auth: &str) -> Result<String, RustyError> {
    let host = var_or_default("SERVER_HOST", "localhost".to_string());
    let port = var_or_default("SERVER_PORT", 8000);
    let protocol = var_or_default("SERVER_PROTOCOL", "https".to_string());
    if !["http", "https"].contains(&protocol.as_str()) {
        return Err(RustyError::RequestError(format!(
            "Unsupported protocol: {protocol}"
        )));
    }
    let api_url = format!("{protocol}://{host}:{port}");
    reqwest::Client::new()
        .post(format!("{api_url}/graphql"))
        .header("Content-Type", "application/json")
        .header("Authorization", auth)
        .json(payload)
        .send()
        .await?
        .text()
        .await
        .map_err(|err| RustyError::RequestError(err.to_string()))
}
