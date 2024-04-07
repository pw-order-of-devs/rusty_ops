use crate::errors::RustyError;

/// Extract environment variable
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn var<T: std::str::FromStr>(key: &str) -> Result<T, RustyError> {
    match std::env::var(key) {
        Ok(res) => T::from_str(&res).map_or_else(
            |_| {
                Err(RustyError::EnvVarError(
                    key.to_string(),
                    "Failed parsing the result".to_string(),
                ))
            },
            |res| Ok(res),
        ),
        Err(err) => Err(RustyError::EnvVarError(key.to_string(), format!("{err}"))),
    }
}

/// Extract environment variable or fall back to default one
pub fn var_or_default<T: std::str::FromStr>(key: &str, default: T) -> T {
    var::<T>(key).unwrap_or(default)
}
