use serde_json::json;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::auth::credentials::Credential;
use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use domain::RustyDomainItem;
use rusty_server::services::pipelines as service;

use crate::rusty_server::services::shared;
use crate::utils::db_connect;

#[tokio::test]
async fn get_all_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::get_all(&db_client, &Credential::System, &None, &None).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().len());
}

#[tokio::test]
async fn get_by_id_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::get_by_id(&db_client, &Credential::System, &id).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert!(result.clone().unwrap().is_some());
    assert_eq!(id, result.unwrap().unwrap().id);
}

#[tokio::test]
async fn create_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterPipeline {
            job_id: id.to_string(),
            branch: None,
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_no_job_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterPipeline {
            job_id: "57c38e8b-1845-49f1-874a-1eefe9923456".to_string(),
            branch: None,
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_no_pipeline_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::assign(&db_client, &Credential::System, "uuid", "uuid").await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_pipeline_already_assigned_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::assign(&db_client, &Credential::System, "uuid", &id).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_pipeline_limit_exceeded_test() {
    std::env::set_var("AGENT_MAX_ASSIGNED_JOBS", "0");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::assign(&db_client, &Credential::System, "uuid", &id).await;
    let _ = db.stop().await;
    std::env::set_var("AGENT_MAX_ASSIGNED_JOBS", "1");
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_pipeline_positive_test() {
    std::env::set_var("AGENT_MAX_ASSIGNED_JOBS", "1");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::assign(&db_client, &Credential::System, &id, &agent_id).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn reset_no_pipeline_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::reset(&db_client, &Credential::System, "uuid").await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn reset_pipeline_wrong_status_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::reset(&db_client, &Credential::System, &id).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn reset_positive_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;

    let result = service::reset(&db_client, &Credential::System, &id).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn set_running_no_pipeline_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::set_running(&db_client, &Credential::System, &id, &agent_id).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn set_running_wrong_status_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::set_running(&db_client, &Credential::System, &id, &agent_id).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn set_running_positive_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: uuid::Uuid::new_v4().to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Assigned,
                job_id: id.to_string(),
                agent_id: Some(agent_id.clone()),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap();

    let result = service::set_running(&db_client, &Credential::System, &id, &agent_id).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn finalize_no_pipeline_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::finalize(
        &db_client,
        &Credential::System,
        &id,
        &agent_id,
        PipelineStatus::Success,
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn finalize_wrong_status_test() {
    std::env::set_var("AGENT_MAX_ASSIGNED_JOBS", "1");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::finalize(
        &db_client,
        &Credential::System,
        &id,
        &agent_id,
        PipelineStatus::Success,
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn finalize_positive_test() {
    std::env::set_var("AGENT_MAX_ASSIGNED_JOBS", "1");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let agent_id = shared::create_agent(&db_client).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: uuid::Uuid::new_v4().to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::InProgress,
                job_id: id.to_string(),
                agent_id: Some(agent_id.clone()),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap();

    let result = service::finalize(
        &db_client,
        &Credential::System,
        &id,
        &agent_id,
        PipelineStatus::Success,
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_by_id_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let id = shared::create_pipeline(&db_client, &id).await;

    let result = service::delete_by_id(&db_client, &Credential::System, &id).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}

#[tokio::test]
async fn delete_many_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::delete_many(
        &db_client,
        &Credential::System,
        &json!({ "job_id": { "equals": &id } }),
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(3, result.unwrap());
}

#[tokio::test]
async fn delete_all_test() {
    std::env::set_var("RUSTY_DEBUG", "true");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let id = shared::create_project(&db_client).await;
    let id = shared::create_job(&db_client, &id).await;
    let _ = shared::create_pipeline(&db_client, &id).await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    std::env::remove_var("RUSTY_DEBUG");
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
