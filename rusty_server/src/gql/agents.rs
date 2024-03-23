use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::agents::{Agent, RegisterAgent};
use domain::filters::search::SearchOptions;
use persist::db_client::DbClient;

use crate::services::agents as service;

pub struct AgentsQuery;

#[Object]
impl AgentsQuery {
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Agent>, RustyError> {
        log::debug!("handling `agents::get` request");
        let entries = service::get_all(ctx.data::<DbClient>()?, filter, options).await?;
        log::debug!("`agents::get`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Agent>, RustyError> {
        log::debug!("handling `agents::getById` request");
        let entry = service::get_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`agents::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct AgentsMutation;

#[Object]
impl AgentsMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        agent: RegisterAgent,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `agents::register` request");
        let id = service::create(ctx.data::<DbClient>()?, agent).await?;
        log::debug!("`agents::register`: created agent with id `{id}`");
        Ok(id)
    }

    async fn healthcheck(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `agents::healthcheck` request");
        let id = service::healthcheck(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`agents::healthcheck`: agent with id `{id}` checked out");
        Ok(id)
    }

    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `agents::deleteById` request");
        let deleted = service::delete_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`agents::deleteById`: deleted agent with id `{id}`");
        Ok(deleted)
    }
}
