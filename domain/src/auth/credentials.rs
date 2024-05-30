use base64_url::base64;
use base64_url::base64::Engine;
use serde_json::{Map, Number, Value};

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
            Self::Bearer(token) => {
                let username = get_token_claim_str(token, "sub");
                write!(f, "{username}")
            }
            Self::None => write!(f, "empty credential"),
        }
    }
}

#[derive(Debug)]
enum ClaimValue {
    Str(String),
    Int(u64),
}

/// Get the claim value as string from the given token.
///
/// # Arguments
///
/// * `token` - A string slice representing the token.
/// * `claim` - A string slice representing the claim to retrieve.
///
/// # Returns
///
/// A `String` containing the value of the claim, if present. If the claim is not found,
/// an empty `String` is returned.
#[must_use]
pub fn get_token_claim_str(token: &str, claim: &str) -> String {
    if let ClaimValue::Str(value) = get_token_claim(token, claim) {
        value
    } else {
        String::new()
    }
}

/// Get the claim value as u64 from the given token.
///
/// # Arguments
///
/// * `token` - A string slice representing the token.
/// * `claim` - A string slice representing the claim name.
///
/// # Returns
///
/// An unsigned 64-bit integer representing the claim value. If the claim value is not found or
/// cannot be converted to u64, a default value of 0 is returned.
#[must_use]
pub fn get_token_claim_u64(token: &str, claim: &str) -> u64 {
    if let ClaimValue::Int(value) = get_token_claim(token, claim) {
        value
    } else {
        0
    }
}

fn get_token_claim(token: &str, claim: &str) -> ClaimValue {
    let claim_value = get_token_claims(token);
    let claim_value = claim_value.get(claim);
    match claim {
        "exp" | "nbf" | "iat" => {
            let value = claim_value
                .unwrap_or(&Value::Number(Number::from(0)))
                .as_u64()
                .unwrap_or(0);
            ClaimValue::Int(value)
        }
        _ => {
            let value = claim_value
                .unwrap_or(&Value::String(String::new()))
                .as_str()
                .unwrap_or("")
                .to_string();
            ClaimValue::Str(value)
        }
    }
}

fn get_token_claims(token: &str) -> Value {
    let claims = token.split('.').collect::<Vec<&str>>();
    if claims.len() == 3 {
        let claims = base64::prelude::BASE64_URL_SAFE_NO_PAD
            .decode(claims[1])
            .unwrap_or(vec![]);
        let claims = String::from_utf8(claims).unwrap_or_default();
        serde_json::from_str::<Value>(&claims).unwrap_or(Value::Object(Map::new()))
    } else {
        Value::Object(Map::new())
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
                log::warn!("malformed auth header");
                return Credential::None;
            }
            Credential::Basic(cred[0].to_string(), cred[1].to_string())
        }
        "Bearer" => {
            let parts = value.split('.').collect::<Vec<&str>>();
            if parts.len() != 3 {
                log::warn!("malformed auth header");
                return Credential::None;
            }
            if base64::prelude::BASE64_URL_SAFE_NO_PAD
                .decode(parts[0])
                .is_err()
                || base64::prelude::BASE64_URL_SAFE_NO_PAD
                    .decode(parts[1])
                    .is_err()
            {
                log::warn!("malformed auth header");
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
