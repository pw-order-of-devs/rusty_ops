use async_graphql::InputObject;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

/// A struct representing a search filter for `get_all` calls.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize)]
pub struct SearchFilter {
    /// Pagination - current page number
    pub page_number: Option<u64>,
    /// Pagination - page size
    pub page_size: Option<u64>,
}

impl Default for SearchFilter {

    fn default() -> Self {
        Self {
            page_number: Some(1),
            page_size: Some(20),
        }
    }
}

impl From<SearchFilter> for FindOptions {

    fn from(value: SearchFilter) -> Self {
        let page_number = value.page_number.unwrap_or(1);
        let page_number = if page_number > 0 { page_number } else { 1 };
        let page_size = value.page_size.unwrap_or(20);
        let mut options = Self::default();
        options.limit = Some(page_size.try_into().unwrap_or(i64::MAX));
        options.skip = Some((page_number - 1) * page_size);
        options
    }
}
