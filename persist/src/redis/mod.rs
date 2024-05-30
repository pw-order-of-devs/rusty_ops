use std::cmp::Ordering;
use std::pin::Pin;
use std::time::Duration;

use bb8_redis::redis::AsyncCommands;
use bb8_redis::{bb8, RedisConnectionManager};
use futures_util::StreamExt;
use serde_json::{json, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

use crate::redis::pubsub::RedisPubSubConnectionManager;
use crate::{Persistence, PersistenceBuilder};

mod pubsub;

/// Represents a `Redis` client.
#[derive(Clone, Debug)]
pub struct RedisClient {
    client: bb8::Pool<RedisConnectionManager>,
    pubsub: bb8::Pool<RedisPubSubConnectionManager>,
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

        let pubsub_manager = RedisPubSubConnectionManager::new(Self::get_conn_string())
            .expect("error while building redis pubsub client");
        let pubsub = bb8::Pool::builder()
            .build(pubsub_manager)
            .await
            .expect("error while building redis client");

        Self { client, pubsub }
    }
}

impl Persistence for RedisClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
        paged: bool,
    ) -> Result<Vec<T>, RustyError> {
        let mut conn = self.client.get().await?;
        let keys: Vec<String> = conn.keys(format!("{index}_*")).await?;

        let mut values: Vec<Value> = vec![];
        for key in keys {
            let value: String = conn.get(key).await?;
            values.push(serde_json::from_str(&value)?);
        }

        let filter = filter.clone().unwrap_or(Value::Null);
        let mut filtered = values
            .into_iter()
            .filter(|item| {
                filter.as_object().map_or(true, |filter| {
                    filter.keys().all(|key| item.get(key) == filter.get(key))
                })
            })
            .collect::<Vec<Value>>();

        let options = options.clone().unwrap_or_default();
        let sort_field = &options.sort_field.unwrap_or_else(|| "id".to_string());
        filtered.sort_by(
            |a, b| match (a[sort_field].clone(), b[sort_field].clone()) {
                (Value::String(a), Value::String(b)) => a.cmp(&b),
                (Value::Number(a), Value::Number(b)) => a
                    .as_f64()
                    .partial_cmp(&b.as_f64())
                    .unwrap_or_else(|| panic!("Failed comparing by {sort_field}")),
                (Value::Bool(a), Value::Bool(b)) => a.cmp(&b),
                _ => Ordering::Equal,
            },
        );
        if options.sort_mode.unwrap_or_default() == SortOptions::Descending {
            filtered.reverse();
        }

        let filtered = filtered
            .into_iter()
            .map(|value| serde_json::from_value(value))
            .collect::<Result<Vec<T>, _>>()?;

        if paged {
            let page_number = usize::try_from(options.page_number.unwrap_or(1))?;
            let page_number = if page_number == 0 { 1 } else { page_number };
            let page_size = usize::try_from(options.page_size.unwrap_or(20))?;
            let page_size = if page_size == 0 { 20 } else { page_size };
            Ok(filtered
                .into_iter()
                .skip((page_number - 1) * page_size)
                .take(page_size)
                .collect())
        } else {
            Ok(filtered)
        }
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        let values: Vec<T> = self.get_all(index, &Some(filter), &None, false).await?;
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
        conn.publish(index, &item).await?;
        Ok(id)
    }

    async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let mut conn = self.client.get().await?;
        let found: Option<T> = self.get_one(index, json!({ "id": id })).await?;
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

    fn change_stream<'a, T: RustyDomainItem + 'static>(
        &'a self,
        index: &'a str,
    ) -> Pin<Box<dyn futures_util::Stream<Item = Option<T>> + Send + 'a>> {
        Box::pin(async_stream::stream! {
            let mut conn = self.pubsub.dedicated_connection().await
                .expect("Error while obtaining redis connection");
            conn.subscribe(index)
                .await
                .unwrap_or_else(|_| panic!("Error while obtaining change stream for `{index}`"));
            while let Some(msg) = conn.on_message().next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if let Ok(item) = serde_json::from_str::<T>(&payload) {
                        yield Some(item)
                    }
                }
            }
        })
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
