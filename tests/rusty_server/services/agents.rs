use testcontainers::runners::AsyncRunner;
use testcontainers_modules::redis::Redis;

use domain::agents::{Agent, RegisterAgent};
use rusty_server::services::agents as service;

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
            "agents",
            &Agent {
                id: "uuid".to_string(),
                expiry: 0,
            },
        )
        .await;

    let result = service::get_all(&db_client, &None, &None).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().len());
}

#[tokio::test]
async fn get_all_paged_test() {
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
                expiry: 0,
            },
        )
        .await;

    let result = service::get_all_paged(&db_client, &None, &None).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().entries.len());
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
            "agents",
            &Agent {
                id: "uuid".to_string(),
                expiry: 0,
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
        RegisterAgent {
            id: "eb083ba6-0a61-4e01-a9a3-8471b8df2ee2".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_limit_exceeded_test() {
    std::env::set_var("AGENTS_REGISTERED_MAX", "0");
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::create(
        &db_client,
        RegisterAgent {
            id: "eb083ba6-0a61-4e01-a9a3-8471b8df2ee2".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    std::env::set_var("AGENTS_REGISTERED_MAX", "24");
    assert!(result.is_err());
}

#[tokio::test]
async fn create_agent_exists_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let _ = db_client
        .create(
            "agents",
            &Agent {
                id: "eb083ba6-0a61-4e01-a9a3-8471b8df2ee2".to_string(),
                expiry: 300,
            },
        )
        .await;

    let result = service::create(
        &db_client,
        RegisterAgent {
            id: "eb083ba6-0a61-4e01-a9a3-8471b8df2ee2".to_string(),
        },
    )
    .await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn healthcheck_test() {
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
                expiry: 0,
            },
        )
        .await;

    let result = service::healthcheck(&db_client, "uuid").await;
    let _ = db.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn healthcheck_no_agent_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;

    let result = service::healthcheck(&db_client, "uuid").await;
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
    let id = db_client
        .create(
            "agents",
            &Agent {
                id: "uuid".to_string(),
                expiry: 0,
            },
        )
        .await;
    let result = service::delete_by_id(&db_client, &id.unwrap()).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}

#[tokio::test]
async fn delete_all_test() {
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
                expiry: 0,
            },
        )
        .await;

    let result = service::delete_all(&db_client).await;
    let _ = db.stop().await;
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());
}
