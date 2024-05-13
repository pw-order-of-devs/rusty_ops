//! Domain module for `rusty_ops`

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

use async_graphql::OutputType;

/// # Agents Module
pub mod agents;

/// # Authentication Module
pub mod auth;

/// # Common Module
pub mod commons;

/// # Jobs Module
pub mod jobs;

/// # Pipelines Module
pub mod pipelines;

/// # Projects Module
pub mod projects;

/// # Template
pub mod templates;

/// The `RustyDomainItem` trait represents an item in a read-only domain.
///
/// It defines the basic requirements that an item must fulfill in order to be considered
/// a valid member of a read-only domain.
///
/// # Implementing `RustyDomainItem`
///
/// To implement the `RustyDomainItem` trait,
/// you need to ensure that your type satisfies all the trait's associated traits.
pub trait RustyDomainItem:
    Send
    + Sync
    + Clone
    + std::fmt::Debug
    + Unpin
    + serde::ser::Serialize
    + for<'de> serde::de::Deserialize<'de>
    + OutputType
{
    /// Returns the identifier of an object.
    fn id(&self) -> String;

    /// Generate a unique identifier using UUID.
    #[must_use]
    fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
