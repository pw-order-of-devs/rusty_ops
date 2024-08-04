use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;

use crate::shared::{delete_one_filter, filter_results, get_value_id, sort_results};
use crate::{Persistence, PersistenceBuilder};

type Store = Arc<Mutex<HashMap<String, HashMap<String, Value>>>>;

/// Represents a `MongoDB` client.
#[derive(Clone, Debug)]
pub struct InMemoryClient {
    store: Store,
}

impl Future for InMemoryClient {
    type Output = Self;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.get_mut().clone())
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for InMemoryClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::from_string("").await
    }

    async fn from_string(_: &str) -> Self {
        tokio::task::spawn_blocking(|| Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        })
        .await
        .expect("error while building in memory client")
    }
}

impl Persistence for InMemoryClient {
    async fn get_all(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<Value>, RustyError> {
        let guarded_store = self.store.lock().unwrap();
        guarded_store.get(index).map_or_else(
            || Ok(vec![]),
            |values| {
                let values = values
                    .iter()
                    .map(|item| item.1)
                    .cloned()
                    .collect::<Vec<Value>>();
                let mut filtered = filter_results(filter, &values);
                let options = options.clone().unwrap_or_default();
                sort_results(&options, &mut filtered);

                Ok(filtered)
            },
        )
    }

    #[allow(clippy::significant_drop_tightening)]
    async fn get_list(&self, index: &str, id: &str) -> Result<Vec<String>, RustyError> {
        let mut guarded_store = self.store.lock().unwrap();
        let index = guarded_store.entry(index.to_string()).or_default();
        let value = index.entry(id.to_string()).or_default();
        let entries = value
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();
        Ok(entries)
    }

    async fn create(&self, index: &str, item: &Value) -> Result<String, RustyError> {
        let id = get_value_id(item);
        {
            let mut guarded_store = self.store.lock().unwrap();
            guarded_store
                .entry(index.to_string())
                .or_default()
                .insert(id.clone(), item.clone());
        }
        let _ = messaging::internal::send(
            &json!({ "index": index, "op": "create", "item": item }).to_string(),
        )
        .await;
        Ok(id)
    }

    async fn update(&self, index: &str, id: &str, item: &Value) -> Result<String, RustyError> {
        let found: Option<Value> = self
            .get_one(index, json!({ "id": { "equals": id } }))
            .await?;
        if found.is_some() {
            let id = self.create(index, item).await?;
            let _ = messaging::internal::send(
                &json!({
                    "index": index,
                    "op": "update",
                    "item": serde_json::to_string(item)?
                })
                .to_string(),
            )
            .await;
            Ok(id)
        } else {
            Err(RustyError::RedisError(format!(
                "Item not found: `{index}`.`{id}`"
            )))
        }
    }

    #[allow(clippy::significant_drop_tightening)]
    async fn append(&self, index: &str, id: &str, entry: &str) -> Result<u64, RustyError> {
        let mut guarded_store = self.store.lock().unwrap();
        let value_array = guarded_store
            .entry(index.to_string())
            .or_default()
            .entry(id.to_string())
            .or_insert_with(|| Value::Array(vec![]));
        if let Value::Array(array) = value_array {
            array.push(Value::String(entry.to_string()));
        }
        Ok(1)
    }

    async fn delete_one(&self, index: &str, filter: Value) -> Result<u64, RustyError> {
        let filter = delete_one_filter(&filter);
        self.get_one(index, filter).await?.map_or(Ok(0), |found| {
            if self
                .store
                .lock()
                .unwrap()
                .get_mut(index)
                .unwrap()
                .remove(
                    found
                        .get("id")
                        .unwrap_or(&Value::Null)
                        .as_str()
                        .unwrap_or_default(),
                )
                .is_some()
            {
                Ok(1)
            } else {
                Ok(0)
            }
        })
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        let mut guarded_store = self.store.lock().unwrap();
        guarded_store
            .remove(index)
            .map_or(Ok(0), |index| Ok(index.len() as u64))
    }

    async fn purge(&self) -> Result<(), RustyError> {
        self.store.lock().unwrap().clear();
        Ok(())
    }
}
