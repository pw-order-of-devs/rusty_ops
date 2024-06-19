use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::projects::{Group, RegisterGroup};
use rusty_server::services::project_groups as service;

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
            "project_groups",
            &Group {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                projects: vec![],
            },
        )
        .await;

    let result = service::get_all(&db_client, &None, &None).await;
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
            "project_groups",
            &Group {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                projects: vec![],
            },
        )
        .await;

    let result = service::get_by_id(&db_client, "uuid").await;
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

    let result = service::create(
        &db_client,
        RegisterGroup {
            name: "sample".to_string(),
        },
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
            "project_groups",
            &Group {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                projects: vec![],
            },
        )
        .await;

    let result = service::delete_by_id(&db_client, "uuid").await;
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
            "project_groups",
            &Group {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                projects: vec![],
            },
        )
        .await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    std::env::remove_var("RUSTY_DEBUG");
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
