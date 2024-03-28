use serde_json::Value;

use commons::errors::RustyError;
use domain::agents::PagedAgents;

use crate::api::client::reqwasm_post;
use crate::api::utils::{parse_entries, parse_paged};

/// Function to retrieve agents from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_agents() -> Result<PagedAgents, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            agents {{
                get(options: {{ pageSize: 24 }}) {{
                    total
                    page
                    pageSize
                    entries {{
                        id
                        expiry
                    }}
                }}
            }}
        }}"#),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["agents"]["get"].clone();
    let (total, page, page_size, entries) = parse_paged(&json_data)?;
    let entries = parse_entries(entries)?;
    Ok(PagedAgents {
        total,
        page,
        page_size,
        entries,
    })
}
