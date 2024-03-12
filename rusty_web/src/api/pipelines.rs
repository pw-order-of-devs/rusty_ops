use commons::errors::RustyError;
use domain::pipelines::{Pipeline, RegisterPipeline};

use crate::api::client::reqwasm_post;
use crate::api::utils::parse_entries;

/// Function to retrieve pipelines for job from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_pipelines_for_job(job_id: String) -> Result<Vec<Pipeline>, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            pipelines {{
                get(
                    filter: {{ job_id: "{}" }},
        			options: {{ sortMode: DESCENDING, sortField: "number" }}
                ) {{
                    id
                    number
                    startDate
                    status
                    jobId
                }}
            }}
        }}"#, job_id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["get"].clone();
    parse_entries(json_data)
}

/// Function to retrieve last pipeline for job from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_last_pipeline_for_job(job_id: String) -> Result<Option<Pipeline>, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"query {{
            pipelines {{
                get(filter: {{
                    job_id: "{}"
                }}, options: {{
                    sortMode: "DESCENDING",
                    sortField: "number",
                    pageSize: 1
                }}) {{
                    id
                    number
                    startDate
                    status
                    jobId
                }}
            }}
        }}"#, job_id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["get"].clone();
    let entries: Vec<Pipeline> = parse_entries(json_data)?;
    Ok(if entries.len() == 1 {
        entries.first().cloned()
    } else {
        None
    })
}

/// Function to run a new pipeline via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn run_pipeline(model: RegisterPipeline) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            pipelines {{
                register(pipeline: {{
                    jobId: "{}"
                }})
            }}
        }}"#, model.job_id),
        "variables": {}
    });

    let data = reqwasm_post(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json_data["data"]["pipelines"]["register"].to_string())
}
