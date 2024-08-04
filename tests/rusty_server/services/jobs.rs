use serde_json::json;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::auth::credentials::Credential;
use domain::jobs::RegisterJob;
use rusty_server::services::jobs as service;

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
    let _ = shared::create_job(&db_client, &id).await;

    let result = service::get_all(&db_client, &Credential::System, &None, &None, &[]).await;
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

    let result = service::get_by_id(&db_client, &Credential::System, &id, &None, &[]).await;
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

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterJob {
            name: "sample".to_string(),
            description: None,
            template: "c3RhZ2VzOgogICB0ZXN0OgogICAgICBzY3JpcHQ6CiAgICAgICAgLSBlY2hvICJoZWxsbyI"
                .to_string(),
            project_id: id,
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_no_project_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterJob {
            name: "sample".to_string(),
            description: None,
            template: "c3RhZ2VzOgogICB0ZXN0OgogICAgICBzY3JpcHQ6CiAgICAgICAgLSBlY2hvICJoZWxsbyI"
                .to_string(),
            project_id: "07fa1b63-1b4b-46a2-8a30-d80440bf6bc3".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
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
    let _ = shared::create_job(&db_client, &id).await;
    let _ = shared::create_job(&db_client, &id).await;
    let _ = shared::create_job(&db_client, &id).await;

    let result = service::delete_many(
        &db_client,
        &Credential::System,
        &json!({ "project_id": { "equals": &id } }),
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
    let _ = shared::create_job(&db_client, &id).await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    std::env::remove_var("RUSTY_DEBUG");
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
