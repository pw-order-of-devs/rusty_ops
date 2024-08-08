use std::time::Duration;

use testcontainers::runners::AsyncRunner;
use testcontainers_modules::rabbitmq::RabbitMq;
use testcontainers_modules::redis::Redis;
use tokio::time::timeout;

use domain::agents::Agent;
use domain::pipelines::{Pipeline, PipelineStatus};
use domain::RustyDomainItem;
use rusty_server::schedulers;

use crate::utils::{db_connect, mq_connect};

#[tokio::test]
async fn schedulers_init_test() {
    let db = Redis
        .start()
        .await
        .expect("initializing test container failed");
    let mq = RabbitMq
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, "redis", 6379).await;
    let mq_client = mq_connect(&mq, "rabbit", 5672).await;
    let _ = schedulers::init(&db_client, &mq_client);
}

#[tokio::test]
async fn scheduler_agent_ttl_test() {
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
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let handle =
        tokio::spawn(async move { schedulers::agent_ttl::schedule(&db_client.clone()).await });
    let result = timeout(Duration::from_secs(1), handle).await;
    let _ = db.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn scheduler_pipelines_cleanup_test() {
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
                start_date: Some("now".to_string()),
                end_date: None,
                status: PipelineStatus::Assigned,
                job_id: "uuid".to_string(),
                agent_id: Some("uuid".to_string()),
            }
            .to_value()
            .unwrap(),
        )
        .await;

    let handle =
        tokio::spawn(
            async move { schedulers::pipeline_cleanup::schedule(&db_client.clone()).await },
        );
    let result = timeout(Duration::from_secs(1), handle).await;
    assert!(result.is_err());
}
