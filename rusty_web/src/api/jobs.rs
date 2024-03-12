use commons::errors::RustyError;
use domain::jobs::{Job, RegisterJob};

use crate::api::client::reqwasm_post;
use crate::api::utils::parse_entries;

/// Function to retrieve jobs for project from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_jobs_for_project(project_id: String) -> Result<Vec<Job>, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            jobs {{
                get(filter: {{
                    project_id: "{}"
                }}) {{
                    id
                    name
                    description
                    template
                    projectId
                }}
            }}
        }}"#, project_id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["jobs"]["get"].clone();
    parse_entries(json_data)
}

/// Function to retrieve a job from a GraphQL endpoint by id.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_job(id: String) -> Result<Job, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            jobs {{
                getById(id: "{}") {{
                    id
                    name
                    description
                    template
                    projectId
                }}
            }}
        }}"#, id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["jobs"]["getById"].clone();
    parse_entries(json_data)
}

/// Function to register a new job via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn register_job(model: RegisterJob) -> Result<String, RustyError> {
    let description = model
        .description
        .map_or_else(String::new, |desc| format!("description: \"{desc}\""));
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            jobs {{
                register(job: {{
                    name: "{}"
                    {}
                    template: "{}"
                    projectId: "{}"
                }})
            }}
        }}"#, model.name, description, model.template, model.project_id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["jobs"]["register"].to_string())
}
