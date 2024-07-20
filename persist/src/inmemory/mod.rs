use serde_json::{json, Value};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;

use crate::messaging::CHANNEL;
use crate::shared::{delete_one_filter, filter_results, sort_results};
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
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let guarded_store = self.store.lock().unwrap();
        if let Some(values) = guarded_store.get(index) {
            let values = values
                .iter()
                .map(|item| item.1)
                .cloned()
                .collect::<Vec<Value>>();
            let mut filtered = filter_results(filter, &values);
            let options = options.clone().unwrap_or_default();
            sort_results(&options, &mut filtered);

            let filtered = filtered
                .into_iter()
                .map(|value| serde_json::from_value(value))
                .collect::<Result<Vec<T>, _>>()?;
            Ok(filtered)
        } else {
            Ok(vec![])
        }
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
        let (id, item) = (item.get_id(), serde_json::to_value(item)?);
        {
            let mut guarded_store = self.store.lock().unwrap();
            if let Some(index) = guarded_store.get_mut(index) {
                index.insert(id.clone(), item.clone());
            } else {
                let mut map = HashMap::new();
                map.insert(id.to_string(), item.clone());
                guarded_store.insert(index.to_string(), map);
            }
        }
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
        let found: Option<T> = self
            .get_one(index, json!({ "id": { "equals": id } }))
            .await?;
        if found.is_some() {
            self.create(id, item).await
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
        let filter = delete_one_filter(&filter);
        self.get_one::<T>(index, filter)
            .await?
            .map_or(Ok(0), |found| {
                if self
                    .store
                    .lock()
                    .unwrap()
                    .get_mut(index)
                    .unwrap()
                    .remove(&found.get_id())
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
