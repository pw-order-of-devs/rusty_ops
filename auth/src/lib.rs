//! Auth module for `rusty_ops`

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

use commons::errors::RustyError;
use domain::auth::credentials::{parse_credential, Credential};
use persist::db_client::DbClient;

/// authenticate user
pub mod authenticate;

/// authorize user
pub mod authorize;

/// jwt token operations
pub mod token;

/// Parses an authentication header and returns a credential if the header is valid.
///
/// # Arguments
///
/// * `header` - value of Authentication header
///
/// # Returns
///
/// * `Some(Credential)` if the header is valid and represents a basic authentication with a username and password.
/// * `None` if the header is invalid or does not represent a basic authentication.
#[must_use]
pub fn parse_auth_header(header: &str) -> Credential {
    let value = header.split(' ').collect::<Vec<&str>>();
    if value.len() != 2 {
        log::warn!("malformed auth header");
        return Credential::None;
    }
    parse_credential(value[0], value[1])
}

/// authenticate user using his credential
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authenticate(db: &DbClient, credential: &Credential) -> Result<String, RustyError> {
    match credential {
        Credential::Basic(user, pass) => authenticate::basic_auth(db, user, pass).await,
        Credential::Bearer(token) => authenticate::bearer_auth(db, token).await,
        Credential::None => Ok(String::new()),
    }
}

/// authenticate user for permission
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authorize(db: &DbClient, username: &str, resources: &str) -> Result<(), RustyError> {
    authorize::authorize(db, username, resources).await
}
