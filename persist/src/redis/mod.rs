use std::time::Duration;

use bb8_redis::redis::AsyncCommands;
use bb8_redis::{bb8, RedisConnectionManager};
use serde_json::{json, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::SearchOptions;

use crate::shared::{delete_one_filter, filter_results, get_value_id, sort_results};
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
    async fn get_all(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<Value>, RustyError> {
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

        Ok(filtered)
    }

    async fn get_list(&self, index: &str, id: &str) -> Result<Vec<String>, RustyError> {
        let mut conn = self.client.get().await?;
        let entries: Vec<String> = conn.lrange(format!("{index}_{id}"), 0, -1).await?;
        Ok(entries)
    }

    async fn create(&self, index: &str, item: &Value) -> Result<String, RustyError> {
        let (id, item) = (get_value_id(item), serde_json::to_string(item)?);
        let mut conn = self.client.get().await?;
        conn.set(format!("{index}_{id}"), &item).await?;
        let _ = messaging::internal::send(
            &json!({ "index": index, "op": "create", "item": item }).to_string(),
        )
        .await;
        Ok(id)
    }

    async fn update(&self, index: &str, id: &str, item: &Value) -> Result<String, RustyError> {
        let item = serde_json::to_string(item)?;
        let mut conn = self.client.get().await?;
        let found = self
            .get_one(index, json!({ "id": { "equals": id } }))
            .await?;
        if found.is_some() {
            conn.set(format!("{index}_{id}"), &item)
                .await?;
            let _ = messaging::internal::send(
                &json!({ "index": index, "op": "update", "item": item }).to_string(),
            )
            .await;
            Ok(id.to_string())
        } else {
            Err(RustyError::RedisError(format!(
                "Item not found: `{index}`.`{id}`"
            )))
        }
    }

    async fn append(&self, index: &str, id: &str, entry: &str) -> Result<u64, RustyError> {
        let mut conn = self.client.get().await?;
        conn.rpush(format!("{index}_{id}"), entry).await?;
        Ok(1)
    }

    async fn delete_one(&self, index: &str, filter: Value) -> Result<u64, RustyError> {
        let mut conn = self.client.get().await?;
        let filter = delete_one_filter(&filter);
        let item: Option<Value> = self.get_one(index, filter).await?;
        if let Some(item) = item {
            conn.del(format!("{index}_{}", get_value_id(&item))).await?;
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
