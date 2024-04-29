use crate::errors::RustyError;
use bcrypt::{hash, verify, DEFAULT_COST};

/// Encodes the given text using bcrypt.
///
/// # Arguments
///
/// * `text` - The text to encode.
///
/// # Returns
///
/// Returns the encoded text as a String if successful, otherwise returns an error.
///
/// # Errors
///
/// Returns a `RustyError` if the encoding fails.
pub fn encode(text: &str) -> Result<String, RustyError> {
    let result = hash(text, DEFAULT_COST)?;
    Ok(result)
}

/// Validates whether the given `text` matches the provided `hash`.
///
/// # Arguments
///
/// * `text` - A `&str` representing the text to verify.
/// * `hash` - A `&str` representing the hash to compare with the text.
///
/// # Returns
///
/// Returns a `Result` indicating whether the verification was successful or not.
/// - If the verification is successful, it returns `Ok(true)`.
/// - If the verification fails or an error occurs, it returns `Err` with a `RustyError`.
///
/// # Errors
///
/// Returns a `RustyError` if the encoding fails.
pub fn validate(text: &str, hash: &str) -> Result<bool, RustyError> {
    let result = verify(text, hash)?;
    Ok(result)
}
