use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;

use crate::inmemory::InMemoryClient;
use crate::mongo::MongoDBClient;
use crate::postgre::PostgreSQLClient;
use crate::redis::RedisClient;
use crate::Persistence;

/// Wrapper for database client
#[derive(Clone, Debug)]
pub enum DbClient {
    /// `DbClient` variant - `InMemory` client
    InMemory(InMemoryClient),
    /// `DbClient` variant - `MongoDb` client
    MongoDb(MongoDBClient),
    /// `DbClient` variant - `PostgreSql` client
    PostgreSql(PostgreSQLClient),
    /// `DbClient` variant - `Redis` client
    Redis(RedisClient),
}

impl DbClient {
    /// Wrapper for `get_all` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get_all(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<Value>, RustyError> {
        match self {
            Self::InMemory(client) => client.get_all(index, filter, options).await,
            Self::MongoDb(client) => client.get_all(index, filter, options).await,
            Self::PostgreSql(client) => client.get_all(index, filter, options).await,
            Self::Redis(client) => client.get_all(index, filter, options).await,
        }
    }

    /// Wrapper for `get_one` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get_one(&self, index: &str, filter: Value) -> Result<Option<Value>, RustyError> {
        match self {
            Self::InMemory(client) => client.get_one(index, filter).await,
            Self::MongoDb(client) => client.get_one(index, filter).await,
            Self::PostgreSql(client) => client.get_one(index, filter).await,
            Self::Redis(client) => client.get_one(index, filter).await,
        }
    }

    /// Wrapper for `get_list` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get_list(&self, index: &str, id: &str) -> Result<Vec<String>, RustyError> {
        match self {
            Self::InMemory(client) => client.get_list(index, id).await,
            Self::MongoDb(client) => client.get_list(index, id).await,
            Self::PostgreSql(client) => client.get_list(index, id).await,
            Self::Redis(client) => client.get_list(index, id).await,
        }
    }

    /// Wrapper for `create` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn create(&self, index: &str, item: &Value) -> Result<String, RustyError> {
        match self {
            Self::InMemory(client) => client.create(index, item).await,
            Self::MongoDb(client) => client.create(index, item).await,
            Self::PostgreSql(client) => client.create(index, item).await,
            Self::Redis(client) => client.create(index, item).await,
        }
    }

    /// Wrapper for `update` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn update(&self, index: &str, id: &str, item: &Value) -> Result<String, RustyError> {
        match self {
            Self::InMemory(client) => client.update(index, id, item).await,
            Self::MongoDb(client) => client.update(index, id, item).await,
            Self::PostgreSql(client) => client.update(index, id, item).await,
            Self::Redis(client) => client.update(index, id, item).await,
        }
    }

    /// Wrapper for `append` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn append(&self, index: &str, id: &str, entry: &str) -> Result<u64, RustyError> {
        match self {
            Self::InMemory(client) => client.append(index, id, entry).await,
            Self::MongoDb(client) => client.append(index, id, entry).await,
            Self::PostgreSql(client) => client.append(index, id, entry).await,
            Self::Redis(client) => client.append(index, id, entry).await,
        }
    }

    /// Wrapper for `delete_one` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn delete_one(&self, index: &str, filter: Value) -> Result<u64, RustyError> {
        match self {
            Self::InMemory(client) => client.delete_one(index, filter).await,
            Self::MongoDb(client) => client.delete_one(index, filter).await,
            Self::PostgreSql(client) => client.delete_one(index, filter).await,
            Self::Redis(client) => client.delete_one(index, filter).await,
        }
    }

    /// Wrapper for `delete_all` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        match self {
            Self::InMemory(client) => client.delete_all(index).await,
            Self::MongoDb(client) => client.delete_all(index).await,
            Self::PostgreSql(client) => client.delete_all(index).await,
            Self::Redis(client) => client.delete_all(index).await,
        }
    }

    /// Purges all data in the selected database(s).
    ///
    /// # Errors
    ///
    /// Returns a `RustyError` if any of the underlying database clients encounter an error during the purge operation.
    pub async fn purge(&self) -> Result<(), RustyError> {
        match self {
            Self::InMemory(client) => client.purge().await,
            Self::MongoDb(client) => client.purge().await,
            Self::PostgreSql(client) => client.purge().await,
            Self::Redis(client) => client.purge().await,
        }
    }
}
