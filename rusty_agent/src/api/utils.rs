use serde::Deserialize;
use serde_json::{from_value, Value};

use commons::errors::RustyError;

/// Parse Server API call result into a type.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn parse_entries<T: std::fmt::Debug + for<'de> Deserialize<'de>>(
    json_data: Value,
) -> Result<T, RustyError> {
    from_value::<T>(json_data).map_err(|err| RustyError::SerializationError {
        message: err.to_string(),
    })
}
