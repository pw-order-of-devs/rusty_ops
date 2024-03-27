use serde_json::Value;

use commons::errors::RustyError;
use domain::agents::{Agent, PagedAgents, RegisterAgent};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;

use crate::services::shared;

const AGENTS_INDEX: &str = "agents";

// query

pub async fn get_all(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<Agent>, RustyError> {
    shared::get_all(db, AGENTS_INDEX, filter, options, false).await
}

pub async fn get_all_paged(
    db: &DbClient,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<PagedAgents, RustyError> {
    let count = shared::get_total_count::<Agent>(db, AGENTS_INDEX, filter).await?;
    let entries = shared::get_all(db, AGENTS_INDEX, filter, options, true).await?;
    let (page, page_size) = shared::to_paged(options)?;
    Ok(PagedAgents {
        total: count,
        page,
        page_size,
        entries,
    })
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Agent>, RustyError> {
    shared::get_by_id(db, AGENTS_INDEX, id).await
}

// mutate

pub async fn create(db: &DbClient, agent: RegisterAgent) -> Result<String, RustyError> {
    if get_by_id(db, &agent.id).await?.is_some() {
        return Err(RustyError::AsyncGraphqlError(format!(
            "agent with id `{}` already exists",
            agent.id
        )));
    }

    shared::create(db, AGENTS_INDEX, agent, |r| Agent::from(&r)).await
}

pub async fn healthcheck(db: &DbClient, id: &str) -> Result<String, RustyError> {
    if let Some(mut agent) = get_by_id(db, id).await? {
        agent.update_expiry();
        db.update(AGENTS_INDEX, id, &agent).await
    } else {
        let message = "`agent::healthcheck` - agent not found".to_string();
        log::debug!("{message}");
        Err(RustyError::AsyncGraphqlError(message))
    }
}

pub async fn delete_by_id(db: &DbClient, id: &str) -> Result<u64, RustyError> {
    shared::delete_by_id::<Agent>(db, AGENTS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, AGENTS_INDEX).await
}
