use serde::Deserialize;
use serde_json::{from_value, Value};

use commons::errors::RustyError;

/// Parse Server API call paged result into a type.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn parse_paged(value: &Value) -> Result<(usize, usize, usize, Value), RustyError> {
    let total = usize::try_from(value["total"].as_u64().unwrap_or(0))?;
    let page = usize::try_from(value["page"].as_u64().unwrap_or(0))?;
    let page_size = usize::try_from(value["pageSize"].as_u64().unwrap_or(0))?;
    let entries = value["entries"].clone();
    Ok((total, page, page_size, entries))
}

/// Parse Server API call result into a type.
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub fn parse_entries<T: for<'de> Deserialize<'de>>(json_data: Value) -> Result<T, RustyError> {
    from_value::<T>(json_data).map_err(|err| RustyError::SerializationError(err.to_string()))
}
