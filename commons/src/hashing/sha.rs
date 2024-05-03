use hmac::{Hmac, Mac};
use sha2::{Digest, Sha512};

use crate::errors::RustyError;

/// Compute the HMAC-SHA512 hash of a given text.
///
/// # Arguments
///
/// * `text` - The text to compute the hash for.
///
/// # Returns
///
/// Returns a `Result` containing an `Hmac<Sha512>` if the hash computation is successful,
/// or a `RustyError` if there is an error during the hashing process.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn hmac512(text: &str) -> Result<Hmac<Sha512>, RustyError> {
    match Hmac::new_from_slice(text.as_bytes()) {
        Ok(text) => Ok(text),
        Err(err) => Err(RustyError::HashingError(err.to_string())),
    }
}

/// Calculates the SHA-512 hash of a given text.
///
/// # Arguments
///
/// * `text` - The text to be hashed.
///
/// # Returns
///
/// A hexadecimal string representation of the SHA-512 hash.
#[must_use]
pub fn sha512(text: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(text);
    let result = hasher.finalize();
    result.iter().fold(String::new(), |mut acc, &value| {
        acc.push_str(&format!("{value:02x}"));
        acc
    })
}
