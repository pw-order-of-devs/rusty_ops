use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::jobs::{Job, PagedJobs, RegisterJob};
use persist::db_client::DbClient;

use crate::gql::get_public_gql_endpoints;
use crate::services::jobs as service;

pub struct JobsQuery;

#[Object]
impl JobsQuery {
    #[auth_macro::authenticate_bearer]
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedJobs, RustyError> {
        log::debug!("handling `jobs::get` request");
        let entries = service::get_all_paged(ctx.data::<DbClient>()?, &filter, &options).await?;
        log::debug!("`jobs::get`: found {} entries", entries.total);
        Ok(entries)
    }

    #[auth_macro::authenticate_bearer]
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
    #[auth_macro::authenticate_bearer]
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

    #[auth_macro::authenticate_bearer]
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

    #[auth_macro::authenticate_bearer]
    async fn delete_all(&self, ctx: &Context<'_>) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `jobs::deleteAll` request");
        let deleted = service::delete_all(ctx.data::<DbClient>()?).await?;
        log::debug!("`jobs::deleteAll`: deleted {deleted} jobs");
        Ok(deleted)
    }
}
