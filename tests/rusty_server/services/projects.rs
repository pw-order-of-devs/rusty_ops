use domain::auth::credentials::Credential;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::projects::{Project, RegisterProject};
use rusty_server::services::projects as service;

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
                group_id: None,
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
                group_id: None,
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

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterProject {
            name: "sample".to_string(),
            url: "http://dummy.ext".to_string(),
            group_id: None,
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_no_group_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterProject {
            name: "sample".to_string(),
            url: "http://dummy.ext".to_string(),
            group_id: Some("uuid".to_string()),
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
    let _ = db_client
        .create(
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                group_id: None,
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
            "projects",
            &Project {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                url: None,
                group_id: None,
            },
        )
        .await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    std::env::remove_var("RUSTY_DEBUG");
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
