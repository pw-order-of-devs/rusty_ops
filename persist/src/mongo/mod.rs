use std::time::Duration;

use futures_util::StreamExt;
use mongodb::bson::{doc, to_document, Document};
use mongodb::options::Credential;
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::{SearchOptions, SortOptions};

use crate::shared::{filter_results, get_value_id};
use crate::{Persistence, PersistenceBuilder};

/// Represents a `MongoDB` client.
#[derive(Clone, Debug)]
pub struct MongoDBClient {
    client: Client,
    database: String,
}

impl MongoDBClient {
    fn get_conn_string() -> String {
        format!(
            "mongodb://{}:{}",
            var_or_default("MONGODB_HOST", "localhost".to_string()),
            var_or_default("MONGODB_PORT", 27017),
        )
    }

    fn configure(client_options: &mut ClientOptions) {
        client_options.credential = Self::get_credential();
        let timeout = var_or_default("DB_CONNECT_TIMEOUT", 30);
        let max_pool_size = var_or_default("DB_POOL_MAX", 24);
        client_options.connect_timeout = Some(Duration::from_secs(timeout));
        client_options.max_pool_size = Some(max_pool_size);
    }

    fn get_credential() -> Option<Credential> {
        match (
            var::<String>("MONGODB_USER"),
            var::<String>("MONGODB_PASSWORD"),
        ) {
            (Ok(user), Ok(pass)) => {
                Some(Credential::builder().username(user).password(pass).build())
            }
            _ => None,
        }
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for MongoDBClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::from_string(&Self::get_conn_string()).await
    }

    async fn from_string(conn: &str) -> Self {
        let mut client_options = ClientOptions::parse(conn)
            .await
            .expect("error while parsing mongodb connection string");
        Self::configure(&mut client_options);
        Self {
            client: Client::with_options(client_options)
                .expect("error while building mongodb client"),
            database: var_or_default("MONGODB_DATABASE", "test".to_string()),
        }
    }
}

impl Persistence for MongoDBClient {
    async fn get_all(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<Value>, RustyError> {
        let mut cursor = self
            .client
            .database(&self.database)
            .collection::<Value>(index)
            .find(parse_options(options))
            .await?;

        let mut result: Vec<Value> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc?);
        }

        Ok(filter_results(filter, &result))
    }

    async fn get_list(&self, index: &str, id: &str) -> Result<Vec<String>, RustyError> {
        let collection = self
            .client
            .database(&self.database)
            .collection::<Value>(index);
        let value = collection
            .find_one(doc! { "id": id })
            .await?
            .unwrap_or_default();
        let entries = value
            .get("entries")
            .unwrap_or(&Value::Array(vec![]))
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();
        Ok(entries)
    }

    async fn create(&self, index: &str, item: &Value) -> Result<String, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Value>(index)
            .insert_one(item)
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))?;
        let _ = messaging::internal::send(
            &json!({ "index": index, "op": "create", "item": serde_json::to_string(item)? })
                .to_string(),
        )
        .await;
        Ok(get_value_id(item))
    }

    async fn update(&self, index: &str, id: &str, item: &Value) -> Result<String, RustyError> {
        if let Some(original) = self
            .get_one(index, json!({ "id": { "equals": id } }))
            .await?
        {
            let id = self
                .client
                .database(&self.database)
                .collection::<Value>(index)
                .update_one(to_document(&original)?, doc! { "$set": to_document(item)? })
                .await
                .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
                .map(|_| {
                    item.get("id")
                        .unwrap_or(&Value::Null)
                        .as_str()
                        .unwrap_or_default()
                })?;
            let _ = messaging::internal::send(
                &json!({ "index": index, "op": "update", "item": serde_json::to_string(item)? })
                    .to_string(),
            )
            .await;
            Ok(id.to_string())
        } else {
            Err(RustyError::MongoDBError(format!(
                "Item not found: `{index}`.`{id}`"
            )))
        }
    }

    async fn append(&self, index: &str, id: &str, entry: &str) -> Result<u64, RustyError> {
        let collection = self
            .client
            .database(&self.database)
            .collection::<Document>(index);

        if collection
            .find_one_and_update(doc! { "id": id }, doc! { "$push": { "entries": entry } })
            .await?
            .is_none()
        {
            collection
                .insert_one(doc! { "id": id, "entries": [entry] })
                .await?;
        }
        Ok(1)
    }

    async fn delete_one(&self, index: &str, filter: Value) -> Result<u64, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Document>(index)
            .delete_one(to_document(&filter)?)
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
            .map(|res| res.deleted_count)
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Document>(index)
            .delete_many(doc! {})
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
            .map(|res| res.deleted_count)
    }

    async fn purge(&self) -> Result<(), RustyError> {
        self.client
            .database(&self.database)
            .drop()
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
    }
}

fn parse_options(options: &Option<SearchOptions>) -> Document {
    options.as_ref().map_or_else(
        || doc! {},
        |value| {
            let sort_mode = value.clone().sort_mode.unwrap_or_default();
            if value.sort_field.is_some() {
                let field = value.clone().sort_field.unwrap();
                let mode = if sort_mode == SortOptions::Ascending {
                    1
                } else {
                    -1
                };
                doc! { field: mode }
            } else {
                doc! {}
            }
        },
    )
}
