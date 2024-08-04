use bb8_lapin::lapin::options::{
    BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions, QueueDeleteOptions,
};
use bb8_lapin::lapin::types::FieldTable;
use bb8_lapin::lapin::{BasicProperties, ConnectionProperties, Consumer};
use bb8_lapin::{bb8, LapinConnectionManager};
use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use futures_lite::StreamExt;
use std::time::Duration;

use crate::mq_consumer::MqConsumer;
use crate::{Consuming, Messaging, MessagingBuilder};

/// Represents a `RabbitMQ` client.
#[derive(Clone, Debug)]
pub struct RabbitMQClient {
    client: bb8::Pool<LapinConnectionManager>,
}

impl RabbitMQClient {
    fn get_conn_string() -> String {
        format!(
            "amqp://{}{}:{}",
            Self::get_credential(),
            var_or_default("RABBITMQ_HOST", "localhost".to_string()),
            var_or_default("RABBITMQ_PORT", 5672),
        )
    }

    fn get_credential() -> String {
        match (
            var::<String>("RABBITMQ_USER"),
            var::<String>("RABBITMQ_PASSWORD"),
        ) {
            (Ok(user), Ok(pass)) => format!("{user}:{pass}@"),
            (_, Ok(pass)) => format!(":{pass}@"),
            _ => String::new(),
        }
    }
}

impl MessagingBuilder for RabbitMQClient {
    type MessagingType = Self;

    async fn build() -> Self {
        Self::from_string(&Self::get_conn_string()).await
    }

    async fn from_string(conn: &str) -> Self {
        let manager = LapinConnectionManager::new(conn, ConnectionProperties::default());

        let timeout = var_or_default("MQ_CONNECT_TIMEOUT", 30);
        let max_pool_size = var_or_default("MQ_POOL_MAX", 24);
        let client = bb8::Pool::builder()
            .connection_timeout(Duration::from_secs(timeout))
            .max_size(max_pool_size)
            .build(manager)
            .await
            .expect("error while building rabbitmq client");

        Self { client }
    }
}

impl Messaging for RabbitMQClient {
    async fn create_queue(&self, name: &str) -> Result<(), RustyError> {
        let conn = self.client.get().await?;
        let channel = conn.create_channel().await?;
        let _ = channel
            .queue_declare(name, QueueDeclareOptions::default(), FieldTable::default())
            .await?;

        Ok(())
    }

    async fn delete_queue(&self, name: &str) -> Result<(), RustyError> {
        let conn = self.client.get().await?;
        let channel = conn.create_channel().await?;
        let _ = channel
            .queue_delete(name, QueueDeleteOptions::default())
            .await?;

        Ok(())
    }

    async fn publish(&self, queue: &str, message: &str) -> Result<(), RustyError> {
        let conn = self.client.get().await?;
        let channel = conn.create_channel().await?;
        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }

    async fn get_consumer(&self, queue: &str) -> Result<MqConsumer, RustyError> {
        let conn = self.client.get().await?;
        match conn
            .create_channel()
            .await?
            .basic_consume(
                queue,
                &uuid::Uuid::new_v4().to_string(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
        {
            Ok(consumer) => Ok(MqConsumer::RabbitMQ(RabbitMQConsumer { consumer })),
            Err(err) => Err(RustyError::MessagingError(err.to_string())),
        }
    }
}

/// Represents a `RabbitMQConsumer` client.
#[derive(Clone, Debug)]
pub struct RabbitMQConsumer {
    consumer: Consumer,
}

impl Consuming for RabbitMQConsumer {
    async fn next(&mut self) -> Option<Result<Vec<u8>, RustyError>> {
        self.consumer.next().await.map(|item| match item {
            Ok(item) => Ok(item.data),
            Err(err) => Err(RustyError::MessagingError(err.to_string())),
        })
    }
}
