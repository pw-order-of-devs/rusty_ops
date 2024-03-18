use std::time::Duration;

use futures_util::StreamExt;
use mongodb::bson::{doc, to_bson, to_document, Bson, Document};
use mongodb::change_stream::event::ChangeStreamEvent;
use mongodb::change_stream::ChangeStream;
use mongodb::options::{Credential, FindOptions};
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Map, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::filters::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

use crate::{Persistence, PersistenceBuilder};

pub use {mongodb::change_stream::event::OperationType, mongodb::error::Error};

/// Represents a `MongoDB` client.
#[derive(Clone, Debug)]
pub struct MongoDBClient {
    database: String,
    client: Client,
}

impl MongoDBClient {
    async fn build_client() -> Self {
        let mut client_options = ClientOptions::parse_async(Self::get_conn_string())
            .await
            .expect("error while parsing mongodb connection string");
        Self::configure(&mut client_options);
        Self {
            database: var("MONGODB_DATABASE").expect("MONGODB_DATABASE variable is required"),
            client: Client::with_options(client_options)
                .expect("error while building mongodb client"),
        }
    }

    fn get_conn_string() -> String {
        format!(
            "mongodb://{}:{}",
            var_or_default("MONGODB_HOST", "localhost".to_string()),
            var_or_default("MONGODB_PORT", 27017),
        )
    }

    fn configure(client_options: &mut ClientOptions) {
        client_options.credential = Some(Self::get_credential());
        client_options.connect_timeout = Some(Duration::new(30, 0));
        client_options.min_pool_size = Some(8);
        client_options.max_pool_size = Some(24);
    }

    fn get_credential() -> Credential {
        Credential::builder()
            .username(var::<String>("MONGODB_USER").expect("MONGODB_USER variable is required"))
            .password(
                var::<String>("MONGODB_PASSWORD").expect("MONGODB_PASSWORD variable is required"),
            )
            .build()
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for MongoDBClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::build_client().await
    }
}

impl Persistence for MongoDBClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let mut cursor = self
            .client
            .database(&self.database)
            .collection::<T>(index)
            .find(parse_filter(&filter)?, parse_options(&options))
            .await?;

        let mut result: Vec<T> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc?);
        }
        Ok(result)
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        self.client
            .database(&self.database)
            .collection::<T>(index)
            .find_one(to_document(&filter)?, None)
            .await?
            .map_or_else(|| Ok(None), |doc| Ok(Some(doc)))
    }

    async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        self.client
            .database(&self.database)
            .collection::<T>(index)
            .insert_one(item, None)
            .await
            .map_err(|err| RustyError::MongoDBError {
                message: err.kind.to_string(),
            })
            .map(|_| item.id())
    }

    async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        if let Some(original) = self.get_one::<T>(index, json!({ "id": id })).await? {
            self.client
                .database(&self.database)
                .collection::<T>(index)
                .update_one(
                    to_document(&original)?,
                    doc! { "$set": to_document(item)? },
                    None,
                )
                .await
                .map_err(|err| RustyError::MongoDBError {
                    message: err.kind.to_string(),
                })
                .map(|_| item.id())
        } else {
            Err(RustyError::MongoDBError {
                message: format!("Item not found: `{index}`.`{id}`"),
            })
        }
    }

    async fn delete_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<u64, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Document>(index)
            .delete_one(to_document(&filter)?, None)
            .await
            .map_err(|err| RustyError::MongoDBError {
                message: err.kind.to_string(),
            })
            .map(|res| res.deleted_count)
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Document>(index)
            .delete_many(doc! {}, None)
            .await
            .map_err(|err| RustyError::MongoDBError {
                message: err.kind.to_string(),
            })
            .map(|res| res.deleted_count)
    }

    async fn change_stream<T: RustyDomainItem>(
        &self,
        index: &str,
    ) -> Result<ChangeStream<ChangeStreamEvent<T>>, mongodb::error::Error> {
        self.client
            .database(&self.database)
            .collection::<T>(index)
            .watch(None, None)
            .await
    }
}

fn parse_filter(filter: &Option<Value>) -> Result<Option<Document>, RustyError> {
    filter.as_ref().map_or_else(
        || Ok(None),
        |value| match to_bson(&value.as_object().unwrap_or(&Map::new()).clone())? {
            Bson::Document(doc) => Ok(Some(doc)),
            _ => Ok(None),
        },
    )
}

fn parse_options(options: &Option<SearchOptions>) -> Option<FindOptions> {
    options.as_ref().map_or_else(
        || None,
        |value| {
            let page_number = value.page_number.unwrap_or(1);
            let page_number = if page_number > 0 { page_number } else { 1 };
            let page_size = value.page_size.unwrap_or(20);
            let sort_mode = value.sort_mode.unwrap_or_default();
            let sort = if value.sort_field.is_some() {
                let field = value.clone().sort_field.unwrap();
                let mode = if sort_mode == SortOptions::Ascending {
                    1
                } else {
                    -1
                };
                Some(doc! { field: mode })
            } else {
                None
            };
            Some(
                FindOptions::builder()
                    .limit(page_size.try_into().unwrap_or(i64::MAX))
                    .skip((page_number - 1) * page_size)
                    .sort(sort)
                    .build(),
            )
        },
    )
}
