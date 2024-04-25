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
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;

/// authenticate user
pub mod authenticate;

/// authenticate user using his credential
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authenticate(db: &DbClient, credential: &Credential) -> Result<User, RustyError> {
    match credential {
        Credential::Basic(user, pass) => authenticate::basic_auth(db, user, pass).await,
    }
}
