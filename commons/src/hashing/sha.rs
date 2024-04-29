use sha2::{Digest, Sha512};

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
