use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

/// A struct representing a search filter for `get_all` calls.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Pagination - current page number
    pub page_number: Option<u64>,
    /// Pagination - page size
    pub page_size: Option<u64>,
}

impl Default for SearchOptions {

    fn default() -> Self {
        Self {
            page_number: Some(1),
            page_size: Some(20),
        }
    }
}
