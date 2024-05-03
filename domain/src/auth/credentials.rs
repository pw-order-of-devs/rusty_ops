use base64_url::base64;
use base64_url::base64::Engine;
use serde_json::{Map, Value};

/// User Credentials
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Credential {
    /// Basic Authentication
    Basic(String, String),
    /// Bearer Authentication
    Bearer(String),
    /// No Authentication
    None,
}

impl std::fmt::Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(user, _) => write!(f, "{user}"),
            Self::Bearer(token) => write!(f, "{}", get_token_username(token)),
            Self::None => write!(f, "empty credential"),
        }
    }
}

/// Extracts the username from a JWT token.
///
/// # Arguments
///
/// * `token` - A string slice representing the JWT token.
///
/// # Returns
///
/// A `String` containing the username extracted from the JWT token. If the token is invalid or
/// the username cannot be extracted, an empty string is returned.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
#[must_use]
pub fn get_token_username(token: &str) -> String {
    let claims = token.split('.').collect::<Vec<&str>>();
    if claims.len() == 3 {
        let claims = base64::prelude::BASE64_URL_SAFE_NO_PAD
            .decode(claims[1])
            .unwrap_or(vec![]);
        let claims = String::from_utf8(claims).unwrap_or_default();
        let claims = serde_json::from_str::<Value>(&claims).unwrap_or(Value::Object(Map::new()));
        claims
            .as_object()
            .unwrap_or(&Map::new())
            .get("sub")
            .unwrap_or(&Value::String(String::new()))
            .as_str()
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    }
}

/// Parses a credential based on the given type and value.
///
/// # Arguments
///
/// * `typ` - The type of the credential.
/// * `value` - The value of the credential.
///
/// # Returns
///
/// The parsed `Credential` object.
#[must_use]
pub fn parse_credential(typ: &str, value: &str) -> Credential {
    match typ {
        "Basic" => {
            let cred = match base64::prelude::BASE64_STANDARD.decode(value) {
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
        }
        "Bearer" => {
            let parts = value.split('.').collect::<Vec<&str>>();
            if parts.len() != 3 {
                log::warn!("invalid auth header: {value:?}");
                return Credential::None;
            }
            if base64::prelude::BASE64_URL_SAFE_NO_PAD
                .decode(parts[0])
                .is_err()
                || base64::prelude::BASE64_URL_SAFE_NO_PAD
                    .decode(parts[1])
                    .is_err()
            {
                log::warn!("invalid auth header: {value:?}");
                return Credential::None;
            };
            Credential::Bearer(value.to_string())
        }
        _ => {
            log::warn!("invalid auth header: unsupported type {typ:?}");
            Credential::None
        }
    }
}
