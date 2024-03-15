use commons::env::var_or_default;

/// HTTP POST request
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `reqwest::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn reqwest_post(payload: &serde_json::Value) -> Result<String, reqwest::Error> {
    let api_url = var_or_default("API_URL", "http://localhost:8000".to_string());
    reqwest::Client::new()
        .post(&format!("{api_url}/graphql"))
        .header("Content-Type", "application/json")
        .json(payload)
        .send()
        .await?
        .text()
        .await
}
