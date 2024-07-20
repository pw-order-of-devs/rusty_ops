use domain::agents::Agent;
use domain::auth::credentials::Credential;
use domain::jobs::Job;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::pipelines::{Pipeline, PipelineStatus, RegisterPipeline};
use domain::projects::Project;
use rusty_server::services::pipelines as service;

use crate::utils::db_connect;

#[tokio::test]
async fn get_all_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::get_by_id(&db_client, &Credential::System, "uuid").await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert!(result.clone().unwrap().is_some());
    assert_eq!("uuid", result.unwrap().unwrap().id);
}

#[tokio::test]
async fn create_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "57c38e8b-1845-49f1-874a-1eefe9923456".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;

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
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Assigned,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::assign(&db_client, &Credential::System, "uuid", "uuid").await;
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
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::assign(&db_client, &Credential::System, "uuid", "uuid").await;
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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::assign(&db_client, &Credential::System, "uuid", "uuid").await;
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
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::reset(&db_client, &Credential::System, "uuid").await;
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
    let _ = db_client
        .create(
            "agents",
            &Agent {
                id: "uuid".to_string(),
                expiry: 300,
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Assigned,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            },
        )
        .await;

    let result = service::reset(&db_client, &Credential::System, "uuid").await;
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

    let result = service::set_running(&db_client, &Credential::System, "uuid", "uuid").await;
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
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            },
        )
        .await;

    let result = service::set_running(&db_client, &Credential::System, "uuid", "uuid").await;
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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Assigned,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            },
        )
        .await;

    let result = service::set_running(&db_client, &Credential::System, "uuid", "uuid").await;
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

    let result = service::finalize(
        &db_client,
        &Credential::System,
        "uuid",
        "uuid",
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
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            },
        )
        .await;

    let result = service::finalize(
        &db_client,
        &Credential::System,
        "uuid",
        "uuid",
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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::InProgress,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            },
        )
        .await;

    let result = service::finalize(
        &db_client,
        &Credential::System,
        "uuid",
        "uuid",
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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                main_branch: "master".to_string(),
                group_id: None,
            },
        )
        .await;
    let _ = db_client
        .create(
            "jobs",
            &Job {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                template: "".to_string(),
                project_id: "uuid".to_string(),
            },
        )
        .await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::delete_by_id(&db_client, &Credential::System, "uuid").await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}

#[tokio::test]
async fn delete_all_test() {
    std::env::set_var("RUSTY_DEBUG", "true");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "pipelines",
            &Pipeline {
                id: "uuid".to_string(),
                number: 0,
                branch: "master".to_string(),
                register_date: "now".to_string(),
                start_date: None,
                end_date: None,
                status: PipelineStatus::Defined,
                job_id: "uuid".to_string(),
                agent_id: None,
            },
        )
        .await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    std::env::remove_var("RUSTY_DEBUG");
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
