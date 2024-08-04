use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::pipelines::{Pipeline, PipelineStatus};
use messaging::mq_client::MqClient;
use messaging::mq_consumer::MqConsumer;
use persist::db_client::DbClient;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

pub async fn schedule(db: &DbClient, mq: &mut MqClient) {
    let timer = var_or_default("SCHEDULER_PIPELINES_LOGS", 1);
    let mut task = tokio::time::interval(Duration::from_secs(timer));

    let mut receiver = messaging::internal::resubscribe().await;
    loop {
        task.tick().await;
        if let Ok(message) = receiver.recv().await {
            handle_pipeline(db, mq, &message).await;
        }
    }
}

async fn handle_pipeline(db: &DbClient, mq: &mut MqClient, message: &str) {
    if let Ok(message) = serde_json::from_str::<Value>(message) {
        let index = message
            .get("index")
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or_default();
        let operation = message
            .get("op")
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or_default();
        let item = message
            .get("item")
            .unwrap_or(&Value::Null)
            .as_str()
            .unwrap_or_default();
        if index == "pipelines" && operation == "update" {
            if let Ok(pipeline) = serde_json::from_str::<Pipeline>(item) {
                if pipeline.status == PipelineStatus::InProgress {
                    if let Ok(mut consumer) = retrieve_consumer(mq, &pipeline.id).await {
                        loop {
                            if let Some(Ok(item)) = consumer.next().await {
                                if let Ok(message) = String::from_utf8(item) {
                                    if &message == "EOF" {
                                        let _ = mq
                                            .delete_queue(&format!("pipeline-logs-{}", pipeline.id))
                                            .await;
                                        return;
                                    } else {
                                        let _ =
                                            db.append("pipelineLogs", &pipeline.id, &message).await;
                                        let _ = messaging::internal::send(
                                            &json!({
                                                "index": "pipelineLogs",
                                                "op": "append",
                                                "id": &pipeline.id,
                                                "entry": &message,
                                            })
                                            .to_string(),
                                        )
                                        .await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn retrieve_consumer(mq: &MqClient, id: &str) -> Result<MqConsumer, RustyError> {
    let mut retries = 0;
    let max_retries = 10;
    let delay_between_retries = Duration::from_millis(500);
    let err = format!("Failed to create a consumer for `pipeline-logs-{id}` queue");

    while retries <= max_retries {
        match mq.get_consumer(&format!("pipeline-logs-{id}")).await {
            Ok(consumer) => {
                log::debug!("Found the requested queue.");
                return Ok(consumer);
            }
            Err(_) => {
                if retries == max_retries {
                    log::debug!("{err}");
                    return Err(RustyError::MessagingError(err));
                } else {
                    log::debug!("{err}. Retrying...");
                    sleep(delay_between_retries).await;
                    retries += 1;
                }
            }
        }
    }

    Err(RustyError::MessagingError(err))
}
