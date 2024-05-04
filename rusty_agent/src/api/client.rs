use commons::env::var_or_default;

/// HTTP POST request with basic authentication
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `reqwest::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn reqwest_post_basic(payload: &serde_json::Value) -> Result<String, reqwest::Error> {
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
pub async fn reqwest_post_bearer(payload: &serde_json::Value) -> Result<String, reqwest::Error> {
    let jwt_token = crate::api::JWT_TOKEN.lock().unwrap().clone();
    reqwest_post(payload, &format!("Bearer {jwt_token}")).await
}

#[allow(clippy::future_not_send)]
async fn reqwest_post(payload: &serde_json::Value, auth: &str) -> Result<String, reqwest::Error> {
    let api_url = var_or_default("API_URL", "http://localhost:8000".to_string());
    reqwest::Client::new()
        .post(format!("{api_url}/graphql"))
        .header("Content-Type", "application/json")
        .header("Authorization", auth)
        .json(payload)
        .send()
        .await?
        .text()
        .await
}
