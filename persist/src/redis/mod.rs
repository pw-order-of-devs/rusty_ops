use std::cmp::Ordering;
use std::fmt::Formatter;

use deadpool_redis::redis::{cmd, AsyncCommands};
use deadpool_redis::{Config, Pool, PoolConfig, Runtime};
use mongodb::change_stream::event::ChangeStreamEvent;
use mongodb::change_stream::ChangeStream;
use serde_json::{json, Value};

use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::filters::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

use crate::{Persistence, PersistenceBuilder};

/// Represents a `Redis` client.
#[derive(Clone)]
pub struct RedisClient {
    pool: Pool,
}

impl std::fmt::Debug for RedisClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RedisClient {:?}", self.pool.manager())
    }
}

impl RedisClient {
    fn build_client() -> Self {
        let mut config = Config::from_url(Self::get_conn_string());
        config.pool = Some(PoolConfig::new(24));

        let pool = config
            .create_pool(Some(Runtime::Tokio1))
            .expect("error while building redis client");
        Self { pool }
    }

    fn get_conn_string() -> String {
        format!(
            "redis://{}:{}",
            var_or_default("REDIS_HOST", "localhost".to_string()),
            var_or_default("REDIS_PORT", 6379),
        )
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for RedisClient {
    type PersistentType = Self;

    async fn build() -> Self {
        async { Self::build_client() }.await
    }
}

impl Persistence for RedisClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let mut conn = self.pool.get().await?;
        let keys: Vec<String> = conn.keys(format!("{index}_*")).await?;

        let mut values: Vec<Value> = vec![];
        for key in keys {
            let value: String = conn.get(key).await?;
            values.push(serde_json::from_str(&value)?);
        }

        let filter = filter.unwrap_or(Value::Null);
        let mut filtered = values
            .into_iter()
            .filter(|item| {
                filter.as_object().map_or(true, |filter| {
                    filter.keys().all(|key| item.get(key) == filter.get(key))
                })
            })
            .collect::<Vec<Value>>();

        let options = options.unwrap_or_default();
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

        let mut page_number = usize::try_from(options.page_number.unwrap_or(1))?;
        if page_number == 0 {
            page_number = 1;
        };
        let page_size = usize::try_from(options.page_size.unwrap_or(20))?;
        Ok(filtered
            .into_iter()
            .skip((page_number - 1) * page_size)
            .take(page_size)
            .collect())
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        let values: Vec<T> = self.get_all(index, Some(filter), None).await?;
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
        let id = item.id();
        let mut conn = self.pool.get().await?;
        let item = serde_json::to_string(item)?;
        conn.set(format!("{index}_{id}"), item).await?;
        cmd("publish")
            .arg(index)
            .arg(&format!("{index}_{id}"))
            .query_async(&mut conn)
            .await?;
        Ok(id)
    }

    async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let mut conn = self.pool.get().await?;
        let found: Option<T> = self.get_one(index, json!({ "id": id })).await?;
        if found.is_some() {
            conn.set(format!("{index}_{id}"), serde_json::to_string(item)?)
                .await?;
            Ok(id.to_string())
        } else {
            Err(RustyError::RedisError {
                message: format!("Item not found: `{index}`.`{id}`"),
            })
        }
    }

    async fn delete_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<u64, RustyError> {
        let mut conn = self.pool.get().await?;
        let item: Option<T> = self.get_one(index, filter).await?;
        if let Some(item) = item {
            conn.del(format!("{index}_{}", item.id())).await?;
            Ok(1)
        } else {
            Ok(0)
        }
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        let mut conn = self.pool.get().await?;
        let keys: Vec<String> = conn.keys(format!("{index}_*")).await?;
        for key in &keys {
            conn.del(key).await?;
        }
        Ok(keys.len() as u64)
    }

    async fn change_stream<T: RustyDomainItem>(
        &self,
        index: &str,
    ) -> Result<ChangeStream<ChangeStreamEvent<T>>, mongodb::error::Error> {
        // let mut conn = self.pool.get().await?;
        // let mut pubsub = conn.into_pubsub();
        // let _ = pubsub.subscribe(index).await?;
        //
        // loop {
        //     let msg = pubsub.on_message()?;
        //
        //     // get the payload as string
        //     let payload : String = msg.get_payload()?;
        //     println!("channel '{}': {}", msg.get_channel_name(), payload);
        // }
        println!("{index}");
        todo!()
    }
}
