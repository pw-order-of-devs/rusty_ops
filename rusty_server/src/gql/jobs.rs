use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::jobs::{Job, RegisterJob};
use persist::db_client::DbClient;

use crate::services::jobs as service;

pub struct JobsQuery;

#[Object]
impl JobsQuery {
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Job>, RustyError> {
        log::debug!("handling `jobs::get` request");
        let entries = service::get_all(ctx.data::<DbClient>()?, filter, options).await?;
        log::debug!("`jobs::get`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Job>, RustyError> {
        log::debug!("handling `jobs::getById` request");
        let entry = service::get_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`jobs::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct JobsMutation;

#[Object]
impl JobsMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        job: RegisterJob,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `jobs::register` request");
        let id = service::create(ctx.data::<DbClient>()?, job).await?;
        log::debug!("`jobs::register`: created job with id `{id}`");
        Ok(id)
    }

    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `jobs::deleteById` request");
        let deleted = service::delete_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`jobs::deleteById`: deleted job with id `{id}`");
        Ok(deleted)
    }
}
