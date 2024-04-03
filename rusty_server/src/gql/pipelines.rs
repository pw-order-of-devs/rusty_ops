use async_graphql::futures_util::Stream;
use async_graphql::{Context, Object, Subscription};
use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::pipelines::{PagedPipelines, Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;

use crate::services::pipelines as service;

pub struct PipelinesQuery;

#[Object]
impl PipelinesQuery {
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedPipelines, RustyError> {
        log::debug!("handling `pipelines::get` request");
        let entries = service::get_all_paged(ctx.data::<DbClient>()?, &filter, &options).await?;
        log::debug!("`pipelines::get`: found {} entries", entries.total);
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Pipeline>, RustyError> {
        log::debug!("handling `pipelines::getById` request");
        let entry = service::get_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`pipelines::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct PipelinesMutation;

#[Object]
impl PipelinesMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        pipeline: RegisterPipeline,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::register` request");
        let id = service::create(ctx.data::<DbClient>()?, pipeline).await?;
        log::debug!("`pipelines::register`: created pipeline with id `{id}`");
        Ok(id)
    }

    async fn assign(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::assign` request");
        let id = service::assign(ctx.data::<DbClient>()?, &pipeline_id, &agent_id).await?;
        log::debug!(
            "`pipelines::assign`: assigned pipeline with id `{pipeline_id}` to agent `{agent_id}`"
        );
        Ok(id)
    }

    async fn set_running(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::setRunning` request");
        let id = service::set_running(ctx.data::<DbClient>()?, &pipeline_id, &agent_id).await?;
        log::debug!("`pipelines::setRunning`: updated pipeline with id `{id}` as `InProgress`");
        Ok(id)
    }

    async fn finalize(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
        status: PipelineStatus,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::finalize` request");
        let id =
            service::finalize(ctx.data::<DbClient>()?, &pipeline_id, &agent_id, status).await?;
        log::debug!("`pipelines::finalize`: updated pipeline with id `{id}` as `{status:?}`");
        Ok(id)
    }
    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `pipelines::deleteById` request");
        let deleted = service::delete_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`pipelines::deleteById`: deleted pipeline with id `{id}`");
        Ok(deleted)
    }

    async fn delete_all(&self, ctx: &Context<'_>) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `pipelines::deleteAll` request");
        let deleted = service::delete_all(ctx.data::<DbClient>()?).await?;
        log::debug!("`pipelines::deleteAll`: deleted {deleted} pipelines");
        Ok(deleted)
    }
}

pub struct PipelineSubscription;

#[Subscription]
impl PipelineSubscription {
    async fn pipelines<'a>(
        &'a self,
        ctx: &Context<'a>,
    ) -> impl Stream<Item = Option<Pipeline>> + 'a {
        log::debug!("handling `pipelines::inserted` subscription");
        let stream = ctx
            .data::<DbClient>()
            .expect("Error while obtaining db client");
        service::inserted_stream(stream)
    }
}
