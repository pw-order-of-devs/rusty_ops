use gloo_net::http::Request;
use serde_json::Value;

use commons::errors::RustyError;
use domain::projects::Project;

/// Function to retrieve projects from a GraphQL endpoint.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[allow(clippy::future_not_send)]
pub async fn get_projects(_: usize) -> Result<Vec<Project>, RustyError> {
    let payload = serde_json::json!({
        "query": "query { getProjects { id name } }",
        "variables": {}
    });

    let data = Request::post("http://localhost:8000/graphql")
        .header("Content-Type", "application/json")
        .json(&payload)?
        .send().await?
        .text().await?;
    let json_data: Value = serde_json::from_str(&data)?;
    serde_json::from_value::<Vec<Project>>(json_data["data"]["getProjects"].clone())
        .map_or(Err(RustyError {}), Ok)
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
        "query": format!("query {{ getProjectById(id: \"{}\") {{ id name url jobs {{ id name description projectId }} }} }}", id),
        "variables": {}
    });

    let data = Request::post("http://localhost:8000/graphql")
        .header("Content-Type", "application/json")
        .json(&payload)?
        .send().await?
        .text().await?;
    let json_data: Value = serde_json::from_str(&data)?;
    serde_json::from_value::<Project>(json_data["data"]["getProjectById"].clone())
        .map_or(Err(RustyError {}), Ok)
}
