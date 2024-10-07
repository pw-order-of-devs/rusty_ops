use domain::agents::Agent;
use domain::jobs::Job;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::projects::{Group, Project, ProjectSource};
use domain::RustyDomainItem;
use persist::db_client::DbClient;
use std::collections::HashMap;

pub(crate) async fn create_agent(db_client: &DbClient) -> String {
    db_client
        .create(
            "agents",
            &Agent {
                id: uuid::Uuid::new_v4().to_string(),
                expiry: 0,
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}

pub(crate) async fn create_project(db_client: &DbClient) -> String {
    db_client
        .create(
            "projects",
            &Project {
                id: uuid::Uuid::new_v4().to_string(),
                source: ProjectSource::Internal,
                name: Some("sample".to_string()),
                url: None,
                main_branch: Some("master".to_string()),
                group_id: None,
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}

pub(crate) async fn create_project_in_group(db_client: &DbClient, id: &str) -> String {
    db_client
        .create(
            "projects",
            &Project {
                id: uuid::Uuid::new_v4().to_string(),
                source: ProjectSource::Internal,
                name: Some("sample".to_string()),
                url: None,
                main_branch: Some("master".to_string()),
                group_id: Some(id.to_string()),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}

pub(crate) async fn create_project_group(db_client: &DbClient) -> String {
    db_client
        .create(
            "project_groups",
            &Group {
                id: uuid::Uuid::new_v4().to_string(),
                name: "sample".to_string(),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}

pub(crate) async fn create_job(db_client: &DbClient, id: &str) -> String {
    db_client
        .create(
            "jobs",
            &Job {
                id: uuid::Uuid::new_v4().to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: id.to_string(),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}

pub(crate) async fn create_pipeline(db_client: &DbClient, id: &str) -> String {
    db_client
        .create(
            "pipelines",
            &Pipeline {
                id: uuid::Uuid::new_v4().to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                stage_status: HashMap::new(),
                status: PipelineStatus::Defined,
                job_id: id.to_string(),
                agent_id: None,
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap()
}
