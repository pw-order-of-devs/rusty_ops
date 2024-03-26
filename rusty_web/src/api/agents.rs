use serde_json::Value;

use commons::errors::RustyError;
use domain::agents::Agent;

use crate::api::client::reqwasm_post;
use crate::api::utils::parse_entries;

/// Function to retrieve agents from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_agents() -> Result<Vec<Agent>, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            agents {{
                get {{
                    id
                    expiry
                }}
            }}
        }}"#),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["agents"]["get"].clone();
    parse_entries(json_data)
}
