use commons::errors::RustyError;

use crate::rabbitmq::RabbitMQConsumer;
use crate::Consuming;

/// Wrapper for messaging client
#[derive(Clone, Debug)]
pub enum MqConsumer {
    /// `MqConsumer` variant - `RabbitMQ` consumer
    RabbitMQ(RabbitMQConsumer),
}

impl MqConsumer {
    /// Wrapper for `next` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn next(&mut self) -> Option<Result<Vec<u8>, RustyError>> {
        match self {
            Self::RabbitMQ(consumer) => consumer.next().await,
        }
    }
}
