use domain::auth::roles::Role;
use domain::auth::user::User;
use testcontainers::runners::AsyncRunner;
use testcontainers::RunnableImage;
use testcontainers_modules::redis::Redis;

use rusty_server::services::roles as service;

use crate::utils::db_connect;

#[tokio::test]
async fn assign_by_id_test() {
    let db = RunnableImage::from(Redis)
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
            },
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
            },
        )
        .await;

    let result = service::assign(&db_client, "uuid", Some("uuid"), None).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assign_by_name_test() {
    let db = RunnableImage::from(Redis)
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
            },
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
            },
        )
        .await;

    let result = service::assign(&db_client, "uuid", None, Some("sample")).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assign_no_user_test() {
    let db = RunnableImage::from(Redis)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::assign(&db_client, "uuid", None, Some("sample")).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assign_no_role_test() {
    let db = RunnableImage::from(Redis)
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
            },
        )
        .await;

    let result = service::assign(&db_client, "uuid", None, None).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}
