use std::time::Duration;
use futures_util::StreamExt;

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{Bson, doc, Document, to_bson, uuid};
use mongodb::options::Credential;

use commons::errors::ROError;
use domain::RODomainItem;

use crate::{Persistence, PersistenceBuilder};

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
        let host = std::env::var("MONGODB_HOST").unwrap_or("localhost".to_string());
        let port = std::env::var("MONGODB_PORT").unwrap_or("27017".to_string());
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
    type PersistentType = MongoDBClient;

    async fn build() -> Self {
        Self::build_client().await
    }
}

impl Persistence for MongoDBClient {

    async fn get_all<T: RODomainItem>(&self, index: &str) -> Result<Vec<T>, ROError> {
        let mut cursor = self.client.database(&self.database)
            .collection::<T>(index).find(None, None).await?;

        let mut result: Vec<T> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc?);
        }
        Ok(result)
    }

    async fn get_by_id<T: RODomainItem>(&self, index: &str, id: &str) -> Result<Option<T>, ROError> {
        let collection = self.client.database(&self.database)
            .collection::<T>(index);

        match collection.find_one(doc! { "id": id }, None).await? {
            Some(doc) => Ok(Some(doc)),
            None => Ok(None),
        }
    }

    async fn create<T: RODomainItem>(&self, index: &str, item: &T) -> Result<String, ROError> {
        let collection = self.client.database(&self.database)
            .collection::<Document>(index);
        let mut document = match to_bson(&item)? {
            Bson::Document(document) => document,
            _ => return Err(ROError {}),
        };

        let id = uuid::Uuid::new().to_string();
        document.insert("id", &id);

        match collection.insert_one(document, None).await {
            Ok(_) => Ok(id),
            Err(_) => Err(ROError {}),
        }
    }

    async fn delete(&self, index: &str, id: &str) -> Result<u64, ROError> {
        let collection = self.client.database(&self.database)
            .collection::<Document>(index);

        match collection.delete_one(doc! { "id": id }, None).await {
            Ok(result) => Ok(result.deleted_count),
            Err(_) => Err(ROError {}),
        }
    }
}
