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

use base64::Engine;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;

/// authenticate user
pub mod authenticate;

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
        log::warn!("invalid auth header: {header}");
        return Credential::None;
    }
    let r#type = value.first().unwrap_or(&"");
    if r#type == &"Basic" {
        let cred = match base64::prelude::BASE64_STANDARD.decode(value[1]) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(decoded) => decoded,
                Err(err) => {
                    log::warn!("invalid auth header: {err}");
                    return Credential::None;
                }
            },
            Err(err) => {
                log::warn!("invalid auth header: {err}");
                return Credential::None;
            }
        };
        let cred = cred.split(':').collect::<Vec<&str>>();
        if cred.len() != 2 {
            log::warn!("invalid auth header: {cred:?}");
            return Credential::None;
        }
        Credential::Basic(cred[0].to_string(), cred[1].to_string())
    } else {
        log::warn!("invalid auth header: unsupported type {:?}", value);
        Credential::None
    }
}

/// authenticate user using his credential
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn authenticate(
    db: &DbClient,
    credential: &Credential,
) -> Result<Option<User>, RustyError> {
    match credential {
        Credential::Basic(user, pass) => authenticate::basic_auth(db, user, pass).await,
        Credential::None => Ok(None),
    }
}
