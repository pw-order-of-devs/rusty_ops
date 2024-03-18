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
use std::pin::Pin;

use serde_json::Value;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::RustyDomainItem;

use crate::db_client::DbClient;
use crate::db_type::DbType;
use crate::mongo::MongoDBClient;
use crate::redis::RedisClient;

/// Wrapper for DB client
pub mod db_client;
mod db_type;

/// # `MongoDB` Module
pub mod mongo;

/// # `Redis` Module
pub mod redis;

/// Defines the `PersistenceBuilder` trait, which is used to construct persistent objects asynchronously.
///
/// The trait provides a method `build()` that returns a future, which eventually produces the constructed object.
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

    /// `build` is a function that builds and constructs an object asynchronously.
    /// It returns a future that will eventually produce the constructed object.
    ///
    /// The returned future is a placeholder for the actual implementation of
    /// constructing the object asynchronously. The output type of the future is
    /// expected to be the same type as the struct on which the `build` function is
    /// defined.
    ///
    /// The `+ Send` constraint on the future indicates that it can be safely sent
    /// across threads.
    ///
    /// # Return Value
    ///
    /// The function returns a future that will produce the constructed object.
    /// The future's `Output` type is the same as the struct that defines the `build`
    /// function.
    fn build() -> impl Future<Output = Self> + Send;
}

/// Defines the Persistence trait which represents a persistence mechanism for storing and retrieving data.
///
/// The trait provides methods for getting all items, getting an item by ID, creating a new item, and deleting an item.
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
    fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> impl Future<Output = Result<Vec<T>, RustyError>> + Send;

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
    fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> impl Future<Output = Result<Option<T>, RustyError>> + Send;

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
    fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
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
    fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> impl Future<Output = Result<String, RustyError>> + Send;

    /// Deletes an item from the database.
    ///
    /// # Arguments
    ///
    /// * `index` - The index name of the item.
    /// * `id` - The unique identifier of the item.
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
    fn delete_one<T: RustyDomainItem>(
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

    /// Fetches a change stream for a collection from the database.
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
    ///
    /// # Notes
    ///
    /// It should return a wrapper or generic result when more databases are supported
    fn change_stream<'a, T: RustyDomainItem + 'static>(
        &'a self,
        index: &'a str,
    ) -> Pin<Box<dyn futures_util::Stream<Item = T> + Send + 'a>>;
}

/// Initializes the persistence layer based on the configured database type.
///
/// Returns an instance of the persistence layer that implements the `Persistence` trait.
pub async fn init() -> DbClient {
    match DbType::parse() {
        DbType::MongoDb => DbClient::MongoDb(MongoDBClient::build().await),
        DbType::Redis => DbClient::Redis(RedisClient::build().await),
    }
}
