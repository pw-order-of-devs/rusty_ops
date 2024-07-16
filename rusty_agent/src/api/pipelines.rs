use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};

use crate::api::client::reqwest_post_bearer;
use crate::api::utils::parse_entries;

/// Function to retrieve one unassigned pipeline from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_unassigned_pipeline() -> Result<Pipeline, RustyError> {
    get_pipeline(r#"{ status: { equals: "defined" } }"#).await
}

/// Function to retrieve last assigned pipeline for agent from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_last_assigned_pipeline(uuid: &str) -> Result<Pipeline, RustyError> {
    get_pipeline(&format!(
        r#"{{ status: {{ equals: "assigned" }}, agent_id: {{ equals: "{uuid}" }} }}"#
    ))
    .await
}

async fn get_pipeline(filter: &str) -> Result<Pipeline, RustyError> {
    let query = format!(
        r#"query {{
            pipelines {{
                get(
                    filter: {filter},
        			options: {{ sortMode: ASCENDING, sortField: "number", pageSize: 1 }}
                ) {{
                    total
                    page
                    pageSize
                    entries {{
                        id
                        number
                        branch
                        startDate
                        registerDate
                        status
                        jobId
                        agentId
                    }}
                }}
            }}
        }}"#
    );
    let payload = serde_json::json!({
        "query": query,
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["get"]["entries"].clone();
    parse_entries::<Vec<Pipeline>>(json_data)?
        .first()
        .map_or_else(
            || Err(RustyError::RequestError("No results".to_string())),
            |pipe| Ok(pipe.clone()),
        )
}

/// Function to assign pipeline to agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn assign_pipeline(pipeline_id: &str, agent_id: &str) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            pipelines {{
                assign(
                    pipelineId: "{}",
                    agentId: "{}"
                )
            }}
        }}"#, pipeline_id, agent_id),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["assign"].clone();
    parse_entries(json_data)
}

/// Function to update pipeline status for agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn set_running(pipeline_id: &str, agent_id: &str) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            pipelines {{
                setRunning(
                    pipelineId: "{}",
                    agentId: "{}"
                )
            }}
        }}"#, pipeline_id, agent_id),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["setRunning"].clone();
    parse_entries(json_data)
}

/// Function to update pipeline final status for agent via GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn finalize(
    pipeline_id: &str,
    agent_id: &str,
    status: PipelineStatus,
) -> Result<String, RustyError> {
    let payload = serde_json::json!({
        "query": format!(r#"mutation {{
            pipelines {{
                finalize(
                    pipelineId: "{}",
                    agentId: "{}",
                    status: {}
                )
            }}
        }}"#, pipeline_id, agent_id, format!("{status:?}").to_uppercase()),
        "variables": {}
    });

    let data = reqwest_post_bearer(&payload).await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    let json_data = json_data["data"]["pipelines"]["finalize"].clone();
    parse_entries(json_data)
}
