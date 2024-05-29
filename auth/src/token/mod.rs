use jwt::claims::SecondsSinceEpoch;
use jwt::header::HeaderType;
use jwt::{AlgorithmType, Claims, Header, RegisteredClaims, SignWithKey, Token};

use commons::errors::RustyError;
use commons::hashing::sha::hmac512;
use domain::auth::user::User;

/// Builds a JSON Web Token (JWT) token with the given user and expiration time.
///
/// # Arguments
///
/// * `user` - The user for whom the token is being built.
/// * `expiry` - The expiration time for the token.
///
/// # Returns
///
/// Returns a `Result` with a `RustyJwtToken` if successful, or a `RustyError` if an error occurs.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn build_jwt_token(user: &User, expiry: u64) -> Result<String, RustyError> {
    let header = build_header();
    let claims = build_claims(&user.username, expiry);
    let token = Token::new(header, claims).sign_with_key(&hmac512(&user.password)?)?;
    Ok(token.as_str().to_string())
}

fn build_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs512,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    }
}

fn build_claims(username: &str, expiry: u64) -> Claims {
    let now: u64 = chrono::Utc::now()
        .timestamp()
        .try_into()
        .unwrap_or_default();
    Claims::new(RegisteredClaims {
        issuer: Some("RustyOps".to_string()),
        subject: Some(username.to_string()),
        audience: Some(username.to_string()),
        issued_at: Some(SecondsSinceEpoch::from(now)),
        not_before: Some(SecondsSinceEpoch::from(now)),
        expiration: Some(SecondsSinceEpoch::from(now + expiry)),
        json_web_token_id: Some(uuid::Uuid::new_v4().to_string()),
    })
}
