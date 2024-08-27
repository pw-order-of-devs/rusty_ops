use serde_json::Value;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::auth::credentials::Credential;
use domain::auth::roles::Role;
use domain::auth::user::User;
use domain::RustyDomainItem;
use rusty_server::services::roles as service;

use crate::utils::db_connect;

#[tokio::test]
async fn assign_by_id_test() {
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
                email: "user@test.org".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
                preferences: Value::Null,
            }
            .to_value()
            .unwrap(),
        )
        .await;
    let _ = db_client
        .create(
            "roles",
            &Role {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                users: vec![],
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::assign(&db_client, &Credential::System, "uuid", Some("uuid"), None).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assign_by_name_test() {
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
                email: "user@test.org".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
                preferences: Value::Null,
            }
            .to_value()
            .unwrap(),
        )
        .await;
    let _ = db_client
        .create(
            "roles",
            &Role {
                id: "uuid".to_string(),
                name: "sample".to_string(),
                description: None,
                users: vec![],
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::assign(
        &db_client,
        &Credential::System,
        "uuid",
        None,
        Some("sample"),
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assign_no_user_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::assign(
        &db_client,
        &Credential::System,
        "uuid",
        None,
        Some("sample"),
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_no_role_test() {
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
                email: "user@test.org".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
                preferences: Value::Null,
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let result = service::assign(&db_client, &Credential::System, "uuid", None, None).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}
