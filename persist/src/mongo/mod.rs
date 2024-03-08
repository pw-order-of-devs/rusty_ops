use std::time::Duration;
use futures_util::StreamExt;

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{Bson, doc, Document, to_bson};
use mongodb::options::{Credential, FindOptions};
use serde_json::{Map, Value};

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::RustyDomainItem;

use crate::{Persistence, PersistenceBuilder};

/// Represents a `MongoDB` client.
#[derive(Debug)]
pub struct MongoDBClient {
    database: String,
    client: Client,
}

impl MongoDBClient {

    async fn build_client() -> Self {
        let mut client_options = ClientOptions::parse(Self::get_conn_string())
            .await.expect("error while parsing mongodb connection string");
        client_options.credential = Some(Self::get_credential());
        client_options.connect_timeout = Some(Duration::new(30,  0));
        client_options.min_pool_size = Some(8);
        client_options.max_pool_size = Some(24);
        Self {
            database: std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE variable is required"),
            client: Client::with_options(client_options).expect("error while building mongodb client"),
        }
    }

    fn get_conn_string() -> String {
        let host = std::env::var("MONGODB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("MONGODB_PORT").unwrap_or_else(|_| "27017".to_string());
        format!("mongodb://{host}:{port}")
    }

    fn get_credential() -> Credential {
        let user = std::env::var("MONGODB_USER")
            .expect("MONGODB_USER variable is required");
        let pass = std::env::var("MONGODB_PASSWORD")
            .expect("MONGODB_PASSWORD variable is required");
        Credential::builder()
            .username(user)
            .password(pass)
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
        let mut cursor = self.client.database(&self.database)
            .collection::<T>(index)
            .find(parse_filter(&filter)?, parse_options(&options))
            .await?;

        let mut result: Vec<T> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc?);
        }
        Ok(result)
    }

    async fn get_by_id<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
    ) -> Result<Option<T>, RustyError> {
        let collection = self.client.database(&self.database)
            .collection::<T>(index);

        collection.find_one(doc! { "id": id }, None).await?
            .map_or_else(|| Ok(None), |doc| Ok(Some(doc)))
    }

    async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let collection = self.client.database(&self.database)
            .collection::<Document>(index);
        let Bson::Document(document) = to_bson(&item)?
            else { return Err(RustyError {}) };

        match collection.insert_one(document, None).await {
            Ok(_) => Ok(item.id()),
            Err(_) => Err(RustyError {}),
        }
    }

    async fn delete(
        &self,
        index: &str,
        id: &str,
    ) -> Result<u64, RustyError> {
        let collection = self.client.database(&self.database)
            .collection::<Document>(index);

        collection.delete_one(doc! { "id": id }, None).await
            .map_or(Err(RustyError {}), |result| Ok(result.deleted_count))
    }
}

fn parse_filter(filter: &Option<Value>) -> Result<Option<Document>, RustyError> {
    match filter {
        None => Ok(None),
        Some(value) => {
            match to_bson(&value.as_object().unwrap_or(&Map::new()).clone())? {
                Bson::Document(doc) => Ok(Some(doc)),
                _ => Ok(None),
            }
        }
    }
}

fn parse_options(options: &Option<SearchOptions>) -> Option<FindOptions> {
    match options {
        None => None,
        Some(value) => {
            let page_number = value.page_number.unwrap_or(1);
            let page_number = if page_number > 0 { page_number } else { 1 };
            let page_size = value.page_size.unwrap_or(20);
            let mut options = FindOptions::default();
            options.limit = Some(page_size.try_into().unwrap_or(i64::MAX));
            options.skip = Some((page_number - 1) * page_size);
            Some(options)
        }
    }
}
