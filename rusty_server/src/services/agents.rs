use serde_json::Value;

use commons::env::var_or_default;
use commons::errors::RustyError;
use domain::agents::{Agent, RegisterAgent};
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;

use crate::services::shared;
use crate::services::shared::get_username_claim;

const AGENTS_INDEX: &str = "agents";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Agent>, RustyError> {
    let entries = shared::get_all::<Agent>(db, AGENTS_INDEX, filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if auth::authorize(db, &username, &format!("AGENTS:READ:ID[{}]", entry.id))
            .await
            .is_ok()
        {
            filtered.push(entry);
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<Option<Agent>, RustyError> {
    auth::authorize(
        db,
        &get_username_claim(cred)?,
        &format!("AGENTS:READ:ID[{}]", id),
    )
    .await?;
    shared::get_by_id(db, AGENTS_INDEX, id).await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    agent: RegisterAgent,
) -> Result<String, RustyError> {
    auth::authorize(db, &get_username_claim(cred)?, "AGENTS:WRITE").await?;

    let max_agents = var_or_default("AGENTS_REGISTERED_MAX", 24);
    if get_all(db, cred, &None, &None).await?.len() >= max_agents {
        return Err(RustyError::AsyncGraphqlError(format!(
            "Exceeded maximum number of registered agents: {max_agents}"
        )));
    }

    if get_by_id(db, cred, &agent.id).await?.is_some() {
        return Err(RustyError::AsyncGraphqlError(format!(
            "agent with id `{}` already exists",
            agent.id
        )));
    }

    shared::create(db, AGENTS_INDEX, agent, |r| {
        Agent::from(&r, var_or_default("AGENT_TTL", 300))
    })
    .await
}

pub async fn healthcheck(db: &DbClient, cred: &Credential, id: &str) -> Result<String, RustyError> {
    if let Some(mut agent) = get_by_id(db, cred, id).await? {
        agent.update_expiry(var_or_default("AGENT_TTL", 300));
        db.update(AGENTS_INDEX, id, &agent).await
    } else {
        let message = "`agent::healthcheck` - agent not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    auth::authorize(db, &get_username_claim(cred)?, "AGENTS:WRITE").await?;
    shared::delete_by_id::<Agent>(db, AGENTS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, AGENTS_INDEX).await
}
