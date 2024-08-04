use commons::errors::RustyError;

use crate::mq_consumer::MqConsumer;
use crate::rabbitmq::RabbitMQClient;
use crate::Messaging;

/// Wrapper for messaging client
#[derive(Clone, Debug)]
pub enum MqClient {
    /// `MqClient` variant - `RabbitMQ` client
    RabbitMQ(RabbitMQClient),
}

impl MqClient {
    /// Wrapper for `create_queue` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn create_queue(&self, name: &str) -> Result<(), RustyError> {
        match self {
            Self::RabbitMQ(client) => client.create_queue(name).await,
        }
    }

    /// Wrapper for `delete_queue` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn delete_queue(&self, name: &str) -> Result<(), RustyError> {
        match self {
            Self::RabbitMQ(client) => client.delete_queue(name).await,
        }
    }

    /// Wrapper for `publish` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn publish(&self, queue: &str, message: &str) -> Result<(), RustyError> {
        match self {
            Self::RabbitMQ(client) => client.publish(queue, message).await,
        }
    }

    /// Wrapper for `get_consumer` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get_consumer(&self, queue: &str) -> Result<MqConsumer, RustyError> {
        match self {
            Self::RabbitMQ(client) => client.get_consumer(queue).await,
        }
    }
}
