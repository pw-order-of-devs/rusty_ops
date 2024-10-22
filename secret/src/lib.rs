//! Secret module for `rusty_ops`

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

use crate::sc_client::ScClient;
use crate::vault::VaultClient;
use commons::errors::RustyError;
use std::future::Future;

/// Wrapper for SC client
pub mod sc_client;

/// # `Vault` Module
pub mod vault;

/// `SecretBuilder` trait definition.
#[allow(opaque_hidden_inferred_bound)]
pub trait SecretBuilder {
    /// The `SecretType` trait is used to define the behavior of secret objects.
    ///
    /// A type that implements `SecretType` must also implement the `Secret` trait, which provides
    /// methods to save and load the object from a storage.
    ///
    /// # Safety
    ///
    /// Implementors of this trait must ensure that the storage operations are safe and follow the necessary
    /// safety requirements for the underlying storage technology.
    type SecretType: Secret;

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

/// Defines the Secret trait which represents a secret mechanism for storing and retrieving data.
pub trait Secret: Send + Sync {
    /// Retrieves the value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the key to lookup.
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
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<String>, RustyError>> + Send;

    /// Puts a value into the storage with the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which the value will be stored.
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
    fn put(&self, key: &str, value: &str) -> impl Future<Output = Result<(), RustyError>> + Send;

    /// Deletes a value into the storage with the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for which the value will be deleted.
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
    fn delete(&self, key: &str) -> impl Future<Output = Result<u64, RustyError>> + Send;
}

/// Initializes the secret layer.
///
/// Returns an instance of the secret layer that implements the `Secret` trait.
pub async fn init() -> ScClient {
    ScClient::Vault(VaultClient::build().await)
}
