use gloo_net::{http::Request, Error};

fn get_api_url() -> String {
    std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
}

/// HTTP POST request
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `gloo_net::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn gloo_post(payload: &serde_json::Value) -> Result<String, Error> {
    Request::post(&format!("{}/graphql", get_api_url()))
        .header("Content-Type", "application/json")
        .json(payload)?
        .send()
        .await?
        .text()
        .await
}
