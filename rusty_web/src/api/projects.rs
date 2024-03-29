use serde_json::Value;

use commons::errors::RustyError;
use domain::projects::{PagedProjects, Project, RegisterProject};

use crate::api::client::reqwasm_post;
use crate::api::utils::{parse_entries, parse_paged};

/// Function to retrieve projects from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_projects() -> Result<PagedProjects, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            projects {{
                get(options: {{ pageSize: 99 }}) {{
                    total
                    page
                    pageSize
                    entries {{
                        id
                        name
                        url
                    }}
                }}
            }}
        }}"#),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["projects"]["get"].clone();
    let (total, page, page_size, entries) = parse_paged(&json_data)?;
    let entries = parse_entries(entries)?;
    Ok(PagedProjects {
        total,
        page,
        page_size,
        entries,
    })
}

/// Function to retrieve a project from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_project(id: String) -> Result<Project, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            projects {{
                getById(id: "{}") {{
                    id
                    name
                    url
                }}
            }}
        }}"#, id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["projects"]["getById"].clone();
    parse_entries(json_data)
}

/// Function to register a new project via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn register_project(model: RegisterProject) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            projects {{
                register(project: {{
                    name: "{}",
                    url: "{}"
                }})
            }}
        }}"#, model.name, model.url),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["projects"]["register"].to_string())
}
