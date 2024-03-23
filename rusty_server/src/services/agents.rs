use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::agents::{Agent, RegisterAgent};
use domain::filters::search::SearchOptions;
use persist::db_client::DbClient;

const AGENTS_INDEX: &str = "agents";

// query

pub async fn get_all(
    db: &DbClient,
    filter: Option<Value>,
    options: Option<SearchOptions>,
) -> Result<Vec<Agent>, RustyError> {
    let entries = db
        .get_all(AGENTS_INDEX, filter, options)
        .await
        .map_err(|err| {
            log::error!("`agents::get`: {err}");
            err
        })?;
    Ok(entries)
}

pub async fn get_by_id(db: &DbClient, id: &str) -> Result<Option<Agent>, RustyError> {
    let entry = db
        .get_one::<Agent>(AGENTS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`agents::getById`: {err}");
            err
        })?;
    Ok(entry)
}

// mutate

pub async fn create(db: &DbClient, agent: RegisterAgent) -> Result<String, RustyError> {
    agent.validate().map_err(|err| {
        log::error!("`agents::create`: {err}");
        err
    })?;

    let agent = Agent::from(&agent);
    if get_by_id(db, &agent.id).await?.is_some() {
        return Err(RustyError::AsyncGraphqlError(format!(
            "agent with id `{}` already exists",
            agent.id
        )));
    }

    let id = db.create(AGENTS_INDEX, &agent).await.map_err(|err| {
        log::error!("`agents::create`: {err}");
        err
    })?;
    Ok(id)
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
    let id = db
        .delete_one::<Agent>(AGENTS_INDEX, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`agents::deleteById`: {err}");
            err
        })?;
    Ok(id)
}
