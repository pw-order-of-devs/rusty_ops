//! Persistence module for `rusty_ops`

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, deny(rust_2018_idioms))]

use std::future::Future;

use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;

use crate::db_client::DbClient;
use crate::db_type::DbType;
use crate::inmemory::InMemoryClient;
use crate::mongo::MongoDBClient;
use crate::postgre::PostgreSQLClient;
use crate::redis::RedisClient;

/// Wrapper for DB client
pub mod db_client;

/// Wrapper for DB type
mod db_type;

/// # `InMemory` Module
pub mod inmemory;

/// # `MongoDB` Module
pub mod mongo;

/// # `PostgreSQL` Module
pub mod postgre;

/// # `Redis` Module
pub mod redis;

/// # `Commons` Module - shared functions
pub mod shared;

/// `PersistenceBuilder` trait definition.
#[allow(opaque_hidden_inferred_bound)]
pub trait PersistenceBuilder {
    /// The `PersistentType` trait is used to define the behavior of persistent objects.
    ///
    /// A type that implements `PersistentType` must also implement the `Persistence` trait, which provides
    /// methods to save and load the object from a storage.
    ///
    /// # Safety
    ///
    /// Implementors of this trait must ensure that the storage operations are safe and follow the necessary
    /// safety requirements for the underlying storage technology.
    type PersistentType: Persistence;

    /// Builds an instance of `Self` asynchronously.
    ///
    /// # Returns
    /// An implementation of `Future` that resolves to `Self` once the build process is complete.
    fn build() -> impl Future<Output = Self> + Send;

    /// Creates a new instance of `Self` from a string representation of a connection.
    ///
    /// # Arguments
    ///
    /// * `conn` - The string representation of a connection.
    ///
    /// # Returns
    ///
    /// A future that resolves to a new instance of `Self`.
    fn from_string(conn: &str) -> impl Future<Output = Self> + Send;
}

/// Defines the Persistence trait which represents a persistence mechanism for storing and retrieving data.
pub trait Persistence: Send + Sync {
    /// Retrieves a list of items by index.
    ///
    /// This method attempts to retrieve n list of items of type `T` from the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index to search in.
    /// * `filter` - Search filter for filtering the results.
    /// * `options` - Sorting/Pagination options.
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn get_all(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> impl Future<Output = Result<Vec<Value>, RustyError>> + Send;

    /// Retrieves an item by index and filter document.
    ///
    /// This method attempts to retrieve an item of type `T` from the specified index using the provided ID.
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index to search in.
    /// * `filter` - The ID of the item to retrieve.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn get_one(
        &self,
        index: &str,
        filter: Value,
    ) -> impl Future<Output = Result<Option<Value>, RustyError>> + Send
    where
        Self: Sized,
    {
        shared::get_one(self, index, filter)
    }

    /// Retrieves a list item by index and filter document.
    ///
    /// This method attempts to retrieve an item of type `Vec<String>` from the specified index using the provided ID.
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index to search in.
    /// * `id` - The ID of the item to retrieve.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn get_list(
        &self,
        index: &str,
        id: &str,
    ) -> impl Future<Output = Result<Vec<String>, RustyError>> + Send;

    /// Creates a new item in the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index where the item will be created.
    /// * `item` - The item to be created in the index.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn create(
        &self,
        index: &str,
        item: &Value,
    ) -> impl Future<Output = Result<String, RustyError>> + Send;

    /// Updates an item in the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index where the item will be modified.
    /// * `id` - The id of the item to be modified.
    /// * `item` - The item to be modified in the index.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn update(
        &self,
        index: &str,
        id: &str,
        item: &Value,
    ) -> impl Future<Output = Result<String, RustyError>> + Send;

    /// Appends a list in the specified index. Works with strings
    ///
    /// # Arguments
    ///
    /// * `index` - The name of the index where the item will be modified.
    /// * `id` - The id of the item to be modified.
    /// * `entry` - string entry to append.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn append(
        &self,
        index: &str,
        id: &str,
        entry: &str,
    ) -> impl Future<Output = Result<u64, RustyError>> + Send;

    /// Deletes an item from the database.
    ///
    /// # Arguments
    ///
    /// * `index` - The index name of the item.
    /// * `id` - The unique identifier of the item.
    /// * `filter` - The ID of the item to retrieve.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn delete_one(
        &self,
        index: &str,
        filter: Value,
    ) -> impl Future<Output = Result<u64, RustyError>> + Send;

    /// Deletes all items from the database.
    ///
    /// # Arguments
    ///
    /// * `index` - The index name of the item.
    ///
    /// # Returns
    ///
    /// A future that resolves to a `Result` indicating whether the operation was successful or returned an error.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    fn delete_all(&self, index: &str) -> impl Future<Output = Result<u64, RustyError>> + Send;

    /// Purges all data in the selected database(s).
    ///
    /// # Errors
    ///
    /// Returns a `RustyError` if any of the underlying database clients encounter an error during the purge operation.
    fn purge(&self) -> impl Future<Output = Result<(), RustyError>> + Send;
}

/// Initializes the persistence layer based on the configured database type.
///
/// Returns an instance of the persistence layer that implements the `Persistence` trait.
pub async fn init() -> DbClient {
    match DbType::parse() {
        DbType::InMemory => DbClient::InMemory(InMemoryClient::build().await),
        DbType::MongoDb => DbClient::MongoDb(MongoDBClient::build().await),
        DbType::PostgreSQL => DbClient::PostgreSql(PostgreSQLClient::build().await),
        DbType::Redis => DbClient::Redis(RedisClient::build().await),
    }
}
