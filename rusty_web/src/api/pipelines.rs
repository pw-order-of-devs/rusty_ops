use serde_json::Value;

use commons::errors::RustyError;
use domain::pipelines::Pipeline;

use crate::api::client::gloo_post;

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
            getPipelines(filter: {{
                job_id: "{}"
            }}) {{
                id
                number
                startDate
                status
                jobId
            }}
        }}"#, job_id),
        "variables": {}
    });

    let data = gloo_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    serde_json::from_value::<Vec<Pipeline>>(json_data["data"]["getPipelines"].clone()).map_err(
        |err| RustyError::SerializationError {
            message: err.to_string(),
        },
    )
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
            getPipelines(filter: {{
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
        }}"#, job_id),
        "variables": {}
    });

    let data = gloo_post(&payload).await?;
    let json_data: Value = serde_json::from_str(&data)?;
    let entries =
        serde_json::from_value::<Vec<Pipeline>>(json_data["data"]["getPipelines"].clone())
            .map_err(|err| RustyError::SerializationError {
                message: err.to_string(),
            })?;
    Ok(if entries.len() == 1 {
        entries.first().cloned()
    } else {
        None
    })
}
