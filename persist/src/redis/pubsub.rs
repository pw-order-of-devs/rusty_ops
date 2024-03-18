use async_trait::async_trait;
use bb8_redis::bb8;
use bb8_redis::redis::aio::PubSub;
use bb8_redis::redis::{Client, IntoConnectionInfo, RedisError};

use commons::errors::RustyError;

#[derive(Clone, Debug)]
pub(crate) struct RedisPubSubConnectionManager {
    client: Client,
}

impl RedisPubSubConnectionManager {
    /// Create a new `RedisPubSubConnectionManager`.
    /// See `redis::Client::open` for a description of the parameter types.
    pub(crate) fn new<T: IntoConnectionInfo>(info: T) -> Result<Self, RustyError> {
        Ok(Self {
            client: Client::open(info.into_connection_info()?)?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisPubSubConnectionManager {
    type Connection = PubSub;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_async_pubsub().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let test_sub_name = "__test__-sub-from-bb8";
        conn.subscribe(test_sub_name).await?;
        conn.unsubscribe(test_sub_name).await
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
