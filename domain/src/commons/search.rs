use async_graphql::{Enum, InputObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An enum representing a sort mode (ascending/descending).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum SortOptions {
    /// Sort results in ascending mode.
    Ascending,
    /// Sort results in descending mode.
    Descending,
}

impl Default for SortOptions {
    fn default() -> Self {
        Self::Ascending
    }
}

/// A struct representing search options for `get_all` calls.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Pagination - current page number
    pub page_number: Option<u64>,
    /// Pagination - page size
    pub page_size: Option<u64>,
    /// Sort mode (ascending/descending)
    pub sort_field: Option<String>,
    /// Sort mode (ascending/descending)
    pub sort_mode: Option<SortOptions>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            page_number: Some(1),
            page_size: Some(20),
            sort_field: None,
            sort_mode: None,
        }
    }
}

/// An enum representing a search filter for `get_all` calls.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SearchFilter {
    /// equals clause
    #[serde(rename(deserialize = "equals"))]
    Equals(Value),
    /// not equals clause
    #[serde(rename(deserialize = "notEquals", deserialize = "not_equals"))]
    NotEquals(Value),
    /// starts with clause
    #[serde(rename(deserialize = "startsWith", deserialize = "starts_with"))]
    StartsWith(Value),
    /// ends with clause
    #[serde(rename(deserialize = "endsWith", deserialize = "ends_with"))]
    EndsWith(Value),
    /// contains clause
    #[serde(rename(deserialize = "contains"))]
    Contains(Value),
    /// greater clause
    #[serde(rename(deserialize = "greaterOrEquals", deserialize = "greater_or_equals"))]
    GreaterOrEqual(Value),
    /// greater than clause
    #[serde(rename(deserialize = "greaterThan", deserialize = "greater_than"))]
    GreaterThan(Value),
    /// less clause
    #[serde(rename(deserialize = "lessOrEquals", deserialize = "less_or_equals"))]
    LessOrEquals(Value),
    /// less than clause
    #[serde(rename(deserialize = "lessThan", deserialize = "less_than"))]
    LessThan(Value),
    /// before clause
    #[serde(rename(deserialize = "before"))]
    Before(Value),
    /// after clause
    #[serde(rename(deserialize = "after"))]
    After(Value),
    /// not before clause
    #[serde(rename(deserialize = "notBefore", deserialize = "not_before"))]
    NotBefore(Value),
    /// not after clause
    #[serde(rename(deserialize = "notAfter", deserialize = "not_after"))]
    NotAfter(Value),
    /// one of many clause
    #[serde(rename(deserialize = "oneOf", deserialize = "one_of"))]
    OneOf(Value),
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self::Equals(Value::Null)
    }
}
