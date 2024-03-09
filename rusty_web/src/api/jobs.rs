use gloo_net::http::Request;
use serde_json::Value;

use commons::errors::RustyError;
use domain::jobs::Job;

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
            getJobs(filter: {{
                project_id: "{}"
            }}) {{
                id
                name
                description
                projectId
            }}
        }}"#, project_id),
        "variables": {}
    });

    let data = Request::post("http://localhost:8000/graphql")
        .header("Content-Type", "application/json")
        .json(&payload)?
        .send()
        .await?
        .text()
        .await?;
    let json_data: Value = serde_json::from_str(&data)?;
    serde_json::from_value::<Vec<Job>>(json_data["data"]["getJobs"].clone())
        .map_err(|err| RustyError::SerializationError { message: err.to_string() })
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
            getJobById(id: "{}") {{
                id
                name
                description
                projectId
            }}
        }}"#, id),
        "variables": {}
    });

    let data = Request::post("http://localhost:8000/graphql")
        .header("Content-Type", "application/json")
        .json(&payload)?
        .send()
        .await?
        .text()
        .await?;
    let json_data: Value = serde_json::from_str(&data)?;
    serde_json::from_value::<Job>(json_data["data"]["getJobById"].clone())
        .map_err(|err| RustyError::SerializationError { message: err.to_string() })
}
