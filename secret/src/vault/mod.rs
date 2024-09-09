use reqwest::Url;
use serde_json::json;

use commons::env::var_or_default;
use commons::errors::RustyError;

use crate::{Secret, SecretBuilder};

/// Represents a `Vault` client.
#[derive(Clone, Debug)]
pub struct VaultClient {
    vault_url: Url,
    client: reqwest::Client,
    token: String,
}

impl VaultClient {
    fn get_conn_string() -> String {
        format!(
            "{}://{}:{}",
            var_or_default("VAULT_PROTOCOL", "https".to_string()),
            var_or_default("VAULT_HOST", "localhost".to_string()),
            var_or_default("VAULT_PORT", 8200),
        )
    }
}

#[allow(clippy::manual_async_fn)]
impl SecretBuilder for VaultClient {
    type SecretType = Self;

    async fn build() -> Self {
        Self::from_string(&Self::get_conn_string()).await
    }

    async fn from_string(conn: &str) -> Self {
        let vault_url = Url::parse(conn).expect("error while parsing vault connection string");
        let client = reqwest::Client::new();
        let token = var_or_default("VAULT_TOKEN", "token".to_string());
        Self {
            vault_url,
            client,
            token,
        }
    }
}

impl Secret for VaultClient {
    async fn get(&self, key: &str) -> Result<Option<String>, RustyError> {
        let response = self
            .client
            .get(format!("{}v1/secret/data/{key}", &self.vault_url))
            .header("X-Vault-Token", &self.token)
            .header("X-Vault-Namespace", "/rusty/ops")
            .send()
            .await?;
        let body = response.text().await?;

        let value: serde_json::Value = serde_json::from_str(&body)?;
        Ok(value["data"]["data"]["key"].as_str().map(String::from))
    }

    async fn put(&self, key: &str, value: &str) -> Result<(), RustyError> {
        let _ = self
            .client
            .post(format!("{}v1/secret/data/{key}", &self.vault_url))
            .header("X-Vault-Token", &self.token)
            .header("X-Vault-Namespace", "/rusty/ops")
            .json(&json!({ "data": { "key": value } }))
            .send()
            .await?;
        Ok(())
    }
}
