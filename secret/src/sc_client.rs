use crate::vault::VaultClient;
use crate::Secret;
use commons::errors::RustyError;

/// Wrapper for secret client
#[derive(Clone, Debug)]
pub enum ScClient {
    /// `DbClient` variant - `InMemory` client
    Vault(VaultClient),
}

impl ScClient {
    /// Wrapper for `get` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn get(&self, key: &str) -> Result<Option<String>, RustyError> {
        match self {
            Self::Vault(client) => client.get(key).await,
        }
    }

    /// Wrapper for `put` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn put(&self, key: &str, value: &str) -> Result<(), RustyError> {
        match self {
            Self::Vault(client) => client.put(key, value).await,
        }
    }

    /// Wrapper for `del` function
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn del(&self, key: &str) -> Result<u64, RustyError> {
        match self {
            Self::Vault(client) => client.delete(key).await,
        }
    }
}
