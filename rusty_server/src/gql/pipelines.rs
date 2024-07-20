use async_graphql::futures_util::Stream;
use async_graphql::{async_stream, Context, Object, Subscription};
use serde_json::Value;

use crate::gql::{get_public_gql_endpoints, shared::paginate};
use crate::services::pipelines as service;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::pipelines::{PagedPipelines, Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;
use persist::messaging::CHANNEL;

pub struct PipelinesQuery;

#[Object]
impl PipelinesQuery {
    #[auth_macro::authenticate(bearer)]
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedPipelines, RustyError> {
        log::debug!("handling `pipelines::get` request");
        let entries = service::get_all(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &filter,
            &options,
        )
        .await?;
        let (total, page, page_size, entries) = paginate(&entries, options);
        log::debug!("`pipelines::get`: found {} entries", total);
        Ok(PagedPipelines {
            total,
            page,
            page_size,
            entries,
        })
    }

    #[auth_macro::authenticate(bearer)]
    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Pipeline>, RustyError> {
        log::debug!("handling `pipelines::getById` request");
        let entry =
            service::get_by_id(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, &id).await?;
        log::debug!("`pipelines::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct PipelinesMutation;

#[Object]
impl PipelinesMutation {
    #[auth_macro::authenticate(bearer)]
    async fn register(
        &self,
        ctx: &Context<'_>,
        pipeline: RegisterPipeline,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::register` request");
        let id =
            service::create(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, pipeline).await?;
        log::debug!("`pipelines::register`: created pipeline with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn assign(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::assign` request");
        let id = service::assign(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &pipeline_id,
            &agent_id,
        )
        .await?;
        log::debug!(
            "`pipelines::assign`: assigned pipeline with id `{pipeline_id}` to agent `{agent_id}`"
        );
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn set_running(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::setRunning` request");
        let id = service::set_running(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &pipeline_id,
            &agent_id,
        )
        .await?;
        log::debug!("`pipelines::setRunning`: updated pipeline with id `{id}` as `InProgress`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn finalize(
        &self,
        ctx: &Context<'_>,
        pipeline_id: String,
        agent_id: String,
        status: PipelineStatus,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `pipelines::finalize` request");
        let id = service::finalize(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &pipeline_id,
            &agent_id,
            status,
        )
        .await?;
        log::debug!("`pipelines::finalize`: updated pipeline with id `{id}` as `{status:?}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `pipelines::deleteById` request");
        let deleted =
            service::delete_by_id(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, &id).await?;
        log::debug!("`pipelines::deleteById`: deleted pipeline with id `{id}`");
        Ok(deleted)
    }

    #[auth_macro::authenticate(bearer)]
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
    async fn pipeline_created(&self) -> impl Stream<Item = Pipeline> {
        log::debug!("handling `pipelines::inserted` subscription");
        let mut receiver = CHANNEL.rx.lock().await;
        async_stream::stream! {
            while let Some(message) = receiver.recv().await {
                if let Ok(message) = serde_json::from_str::<Value>(&message) {
                    let index = message.get("index").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                    let operation = message.get("op").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                    let item = message.get("item").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                    if index == "pipelines" && operation == "create" {
                        if let Ok(pipeline) = serde_json::from_str::<Pipeline>(item) {
                            yield pipeline;
                        }
                    }
                }
            }
        }
    }
}
