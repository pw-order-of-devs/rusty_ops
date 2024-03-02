use std::time::Duration;

use mongodb::{Client, options::ClientOptions};
use mongodb::options::Credential;

use crate::Persistence;

pub struct MongoDBClient {
    _database: String,
    _client: Client,
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
            _database: std::env::var("MONGODB_DATABASE").unwrap_or("".to_string()),
            _client: Client::with_options(client_options).expect("error while building mongodb client"),
        }
    }

    fn get_conn_string() -> String {
        let host = std::env::var("MONGODB_HOST").unwrap_or("localhost".to_string());
        let port = std::env::var("MONGODB_PORT").unwrap_or("27017".to_string());
        format!("mongodb://{host}:{port}")
    }

    fn get_credential() -> Credential {
        let user = std::env::var("MONGODB_USER").expect("MONGODB_USER variable must be set");
        let pass = std::env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD variable must be set");
        Credential::builder()
            .username(user)
            .password(pass)
            .build()
    }
}

#[allow(clippy::manual_async_fn)]
impl Persistence for MongoDBClient {

    async fn build() -> Self {
        Self::build_client().await
    }
}
