use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::auth::credentials::Credential;
use domain::auth::roles::Role;
use domain::auth::user::{RegisterUser, User};
use domain::RustyDomainItem;
use rusty_server::services::users as service;

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
            "users",
            &User {
                id: "uuid".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            }
            .to_value()
            .unwrap(),
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
    let id = db_client
        .create(
            "users",
            &User {
                id: "uuid".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            }
            .to_value()
            .unwrap(),
        )
        .await
        .unwrap();

    let result = service::get_by_id(&db_client, &Credential::System, &id).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert!(result.clone().unwrap().is_some());
    assert_eq!(id, result.unwrap().unwrap().id);
}

#[tokio::test]
async fn get_by_username_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "users",
            &User {
                id: "uuid".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::get_by_username(&db_client, &Credential::System, "user").await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert!(result.clone().unwrap().is_some());
    assert_eq!("user", result.unwrap().unwrap().username);
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
        RegisterUser {
            username: "user".to_string(),
            password: "pass".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_with_role_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "roles",
            &Role {
                id: "uuid".to_string(),
                name: "USERS".to_string(),
                description: None,
                users: vec![],
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterUser {
            username: "user".to_string(),
            password: "pass".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_already_exists_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "users",
            &User {
                id: "uuid".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::create(
        &db_client,
        &Credential::System,
        RegisterUser {
            username: "user".to_string(),
            password: "pass".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}
