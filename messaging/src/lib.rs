//! Messaging module for `rusty_ops`

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, deny(rust_2018_idioms))]

use std::future::Future;

use commons::errors::RustyError;

use crate::mq_client::MqClient;
use crate::mq_consumer::MqConsumer;
use crate::mq_type::MqType;
use crate::rabbitmq::RabbitMQClient;

/// Wrapper for MQ client
pub mod mq_client;

/// Wrapper for MQ consumer
pub mod mq_consumer;

/// Wrapper for MQ type
mod mq_type;

/// # internal channel support Module
#[cfg(feature = "internal")]
pub mod internal;

/// # `RabbitMQ` Module
#[cfg(feature = "external")]
pub mod rabbitmq;

/// `MessagingBuilder` trait definition.
#[allow(opaque_hidden_inferred_bound)]
pub trait MessagingBuilder {
    /// The `MessagingType` trait is used to define the behavior of persistent objects.
    ///
    /// A type that implements `MessagingType` must also implement the `Messaging` trait, which provides
    /// methods to save and load the object from a queue.
    type MessagingType: Messaging;

    /// Builds an instance of `Self` asynchronously.
    ///
    /// # Returns
    /// An implementation of `Future` that resolves to `Self` once the build process is complete.
    fn build() -> impl Future<Output = Self> + Send;

    /// Creates a new instance of `Self` from a string representation of a connection.
    ///
    /// # Arguments
    ///
    /// * `conn` - The string representation of a connection.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of `Self`.
    fn from_string(conn: &str) -> impl Future<Output = Self> + Send;
}

/// Defines the Messaging trait which represents a messaging mechanism for storing and retrieving data in a queue.
pub trait Messaging: Send + Sync {
    /// Declares a queue.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the queue to create.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the operation.
    fn create_queue(&self, name: &str) -> impl Future<Output = Result<(), RustyError>> + Send;

    /// Deletes a queue.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the queue to delete.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the operation.
    fn delete_queue(&self, name: &str) -> impl Future<Output = Result<(), RustyError>> + Send;

    /// Publishes a message to a queue.
    ///
    /// # Arguments
    ///
    /// * `queue` - The name of the queue to publish to.
    /// * `message` - Text to publish.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the operation.
    fn publish(
        &self,
        queue: &str,
        message: &str,
    ) -> impl Future<Output = Result<(), RustyError>> + Send;

    /// Creates a consumer to subscribe to a queue.
    ///
    /// # Arguments
    ///
    /// * `queue` - The name of the queue to subscribe to.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the operation.
    fn get_consumer(
        &self,
        queue: &str,
    ) -> impl Future<Output = Result<MqConsumer, RustyError>> + Send;
}

/// Defines the Consuming trait which represents a support messaging mechanism for subscriptions.
pub trait Consuming: Send + Sync {
    /// Fetch next item from a queue consumer.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn next(&mut self) -> impl Future<Output = Option<Result<Vec<u8>, RustyError>>> + Send;
}

/// Initializes the messaging layer based on the configured messaging type.
///
/// Returns an instance of the persistence layer that implements the `Messaging` trait.
pub async fn init() -> MqClient {
    match MqType::parse() {
        MqType::RabbitMQ => MqClient::RabbitMQ(RabbitMQClient::build().await),
    }
}
