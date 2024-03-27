use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;

use crate::mongo::MongoDBClient;
use crate::postgre::PostgreSQLClient;
use crate::redis::RedisClient;
use crate::Persistence;

/// Wrapper for database client
#[derive(Clone, Debug)]
pub enum DbClient {
    /// DbClient variant - `MongoDb` client
    MongoDb(MongoDBClient),
    /// DbClient variant - `PostgreSql` client
    PostgreSql(PostgreSQLClient),
    /// DbClient variant - `Redis` client
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
    pub async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
        paged: bool,
    ) -> Result<Vec<T>, RustyError> {
        match self {
            Self::MongoDb(client) => client.get_all(index, filter, options, paged).await,
            Self::PostgreSql(client) => client.get_all(index, filter, options, paged).await,
            Self::Redis(client) => client.get_all(index, filter, options, paged).await,
        }
    }

    /// Wrapper for `get_one` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        match self {
            Self::MongoDb(client) => client.get_one(index, filter).await,
            Self::PostgreSql(client) => client.get_one(index, filter).await,
            Self::Redis(client) => client.get_one(index, filter).await,
        }
    }

    /// Wrapper for `create` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        match self {
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
    pub async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        match self {
            Self::MongoDb(client) => client.update(index, id, item).await,
            Self::PostgreSql(client) => client.update(index, id, item).await,
            Self::Redis(client) => client.update(index, id, item).await,
        }
    }

    /// Wrapper for `delete_one` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn delete_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<u64, RustyError> {
        match self {
            Self::MongoDb(client) => client.delete_one::<T>(index, filter).await,
            Self::PostgreSql(client) => client.delete_one::<T>(index, filter).await,
            Self::Redis(client) => client.delete_one::<T>(index, filter).await,
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
            Self::MongoDb(client) => client.delete_all(index).await,
            Self::PostgreSql(client) => client.delete_all(index).await,
            Self::Redis(client) => client.delete_all(index).await,
        }
    }

    /// Wrapper for `change_stream` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub fn change_stream<'a, T: RustyDomainItem + 'static>(
        &'a self,
        index: &'a str,
    ) -> impl futures_util::Stream<Item = T> + 'a {
        match self {
            Self::MongoDb(client) => client.change_stream(index),
            Self::PostgreSql(client) => client.change_stream(index),
            Self::Redis(client) => client.change_stream(index),
        }
    }
}
