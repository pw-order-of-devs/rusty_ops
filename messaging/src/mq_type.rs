use commons::env::var;

/// The `MqType` enum represents the types of messaging supported by the application.
///
/// # Variants
///
/// - `RabbitMQ`: Represents a `RabbitMQ` messaging.
#[derive(Debug)]
pub enum MqType {
    /// Placeholder for no messaging support.
    None,
    /// A `RabbitMQ` client for connecting to a `RabbitMQ` service.
    RabbitMQ,
}

impl MqType {
    /// Parses the `RUSTY_MESSAGING` environment variable and returns the corresponding `MqType` value.
    ///
    /// # Panics
    /// if the `RUSTY_MESSAGING` variable is not set or if the value is not supported.
    ///
    /// # Returns
    /// - `MqType::None` if the `RUSTY_MESSAGING` value is `none`
    /// - `MqType::RabbitMQ` if the `RUSTY_MESSAGING` value is `rabbitmq` or `rabbit`
    #[must_use]
    pub fn parse() -> Self {
        let mq_type = var::<String>("RUSTY_MESSAGING")
            .expect("RUSTY_MESSAGING variable is required")
            .to_lowercase();

        match mq_type.as_str() {
            "none" => Self::None,
            "rabbitmq" | "rabbit" => Self::RabbitMQ,
            _ => panic!("Unsupported messaging: {mq_type}"),
        }
    }
}
