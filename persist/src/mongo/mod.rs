use std::pin::Pin;
use std::time::Duration;

use futures_util::StreamExt;
use mongodb::bson::{doc, to_bson, to_document, Bson, Document};
use mongodb::change_stream::event::OperationType;
use mongodb::options::{Credential, FindOptions};
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Map, Value};

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

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
        let mut client_options = ClientOptions::parse_async(conn)
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
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
        paged: bool,
    ) -> Result<Vec<T>, RustyError> {
        let mut cursor = self
            .client
            .database(&self.database)
            .collection::<T>(index)
            .find(parse_filter(filter)?, parse_options(options, paged))
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
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
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
                .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
                .map(|_| item.id())
        } else {
            Err(RustyError::MongoDBError(format!(
                "Item not found: `{index}`.`{id}`"
            )))
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
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
            .map(|res| res.deleted_count)
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        self.client
            .database(&self.database)
            .collection::<Document>(index)
            .delete_many(doc! {}, None)
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
            .map(|res| res.deleted_count)
    }

    fn change_stream<'a, T: RustyDomainItem + 'static>(
        &'a self,
        index: &'a str,
    ) -> Pin<Box<dyn futures_util::Stream<Item = Option<T>> + Send + 'a>> {
        Box::pin(async_stream::stream! {
            if let Ok(mut change_stream) = self.client
                .database(&self.database)
                .collection::<T>(index)
                .watch(None, None)
                .await {
                while let Some(event) = change_stream.next().await {
                    if let Ok(event) = event {
                        if event.operation_type == OperationType::Insert {
                            if let Some(document) = event.full_document {
                                yield Some(document);
                            }
                        }
                    }
                }
            } else {
                log::trace!("Error while obtaining a change stream for `{index}`: not supported in current server configuration");
                yield None;
            }
        })
    }

    async fn purge(&self) -> Result<(), RustyError> {
        self.client
            .database(&self.database)
            .drop(None)
            .await
            .map_err(|err| RustyError::MongoDBError(err.kind.to_string()))
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

fn parse_options(options: &Option<SearchOptions>, paged: bool) -> Option<FindOptions> {
    options.as_ref().map_or_else(
        || None,
        |value| {
            let page_number = value.page_number.unwrap_or(1);
            let page_number = if page_number == 0 { 1 } else { page_number };
            let page_size = value.page_size.unwrap_or(20);
            let page_size = if page_size == 0 { 20 } else { page_size };
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

            let options = if paged {
                FindOptions::builder()
                    .limit(page_size.try_into().unwrap_or(i64::MAX))
                    .skip((page_number - 1) * page_size)
                    .sort(sort)
                    .build()
            } else {
                FindOptions::builder().sort(sort).build()
            };
            Some(options)
        },
    )
}
