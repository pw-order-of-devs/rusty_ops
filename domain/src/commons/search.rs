use async_graphql::{Enum, InputObject};
use serde::{Deserialize, Serialize};

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

/// A struct representing a search filter for `get_all` calls.
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
