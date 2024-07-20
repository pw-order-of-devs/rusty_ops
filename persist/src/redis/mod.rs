use std::time::Duration;

use bb8_redis::redis::AsyncCommands;
use bb8_redis::{bb8, RedisConnectionManager};
use serde_json::{json, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;

use crate::messaging::CHANNEL;
use crate::shared::{delete_one_filter, filter_results, sort_results};
use crate::{Persistence, PersistenceBuilder};

/// Represents a `Redis` client.
#[derive(Clone, Debug)]
pub struct RedisClient {
    client: bb8::Pool<RedisConnectionManager>,
}

impl RedisClient {
    fn get_conn_string() -> String {
        format!(
            "redis://{}{}:{}",
            Self::get_credential(),
            var_or_default("REDIS_HOST", "localhost".to_string()),
            var_or_default("REDIS_PORT", 6379),
        )
    }

    fn get_credential() -> String {
        match (var::<String>("REDIS_USER"), var::<String>("REDIS_PASSWORD")) {
            (Ok(user), Ok(pass)) => format!("{user}:{pass}@"),
            (_, Ok(pass)) => format!(":{pass}@"),
            _ => String::new(),
        }
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for RedisClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::from_string(&Self::get_conn_string()).await
    }

    async fn from_string(conn: &str) -> Self {
        let manager = RedisConnectionManager::new(conn).expect("error while building redis client");

        let timeout = var_or_default("DB_CONNECT_TIMEOUT", 30);
        let max_pool_size = var_or_default("DB_POOL_MAX", 24);
        let client = bb8::Pool::builder()
            .connection_timeout(Duration::from_secs(timeout))
            .max_size(max_pool_size)
            .build(manager)
            .await
            .expect("error while building redis client");

        Self { client }
    }
}

impl Persistence for RedisClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let mut conn = self.client.get().await?;
        let keys: Vec<String> = conn.keys(format!("{index}_*")).await?;

        let mut values: Vec<Value> = vec![];
        for key in keys {
            let value: String = conn.get(key).await?;
            values.push(serde_json::from_str(&value)?);
        }

        let mut filtered = filter_results(filter, &values);
        let options = options.clone().unwrap_or_default();
        sort_results(&options, &mut filtered);

        let filtered = filtered
            .into_iter()
            .map(|value| serde_json::from_value(value))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(filtered)
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        let values: Vec<T> = self.get_all(index, &Some(filter), &None).await?;
        if values.len() == 1 {
            Ok(Some(values[0].clone()))
        } else {
            Ok(None)
        }
    }

    async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let id = item.get_id();
        let mut conn = self.client.get().await?;
        let item = serde_json::to_string(item)?;
        conn.set(format!("{index}_{id}"), &item).await?;
        let _ = CHANNEL
            .tx
            .lock()
            .await
            .try_send(json!({ "index": index, "op": "create", "item": item }).to_string());
        Ok(id)
    }

    async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let mut conn = self.client.get().await?;
        let found: Option<T> = self
            .get_one(index, json!({ "id": { "equals": id } }))
            .await?;
        if found.is_some() {
            conn.set(format!("{index}_{id}"), serde_json::to_string(item)?)
                .await?;
            Ok(id.to_string())
        } else {
            Err(RustyError::RedisError(format!(
                "Item not found: `{index}`.`{id}`"
            )))
        }
    }

    async fn delete_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<u64, RustyError> {
        let mut conn = self.client.get().await?;
        let filter = delete_one_filter(&filter);
        let item: Option<T> = self.get_one(index, filter).await?;
        if let Some(item) = item {
            conn.del(format!("{index}_{}", item.get_id())).await?;
            Ok(1)
        } else {
            Ok(0)
        }
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        let mut conn = self.client.get().await?;
        let keys: Vec<String> = conn.keys(format!("{index}_*")).await?;
        for key in &keys {
            conn.del(key).await?;
        }
        Ok(keys.len() as u64)
    }

    async fn purge(&self) -> Result<(), RustyError> {
        let mut conn = self.client.get().await?;
        let keys: Vec<String> = conn.keys("*").await?;
        for key in &keys {
            let _: () = conn.del(key).await?;
        }

        Ok(())
    }
}
