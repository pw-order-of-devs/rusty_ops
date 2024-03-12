use reqwasm::{http::Request, Error};

/// HTTP POST request
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `reqwasm::Error` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn reqwasm_post(payload: &serde_json::Value) -> Result<String, Error> {
    Request::post(&format!("{}/graphql", crate::env::APP_API_URL))
        .header("Content-Type", "application/json")
        .body(&payload.to_string())
        .send()
        .await?
        .text()
        .await
}
