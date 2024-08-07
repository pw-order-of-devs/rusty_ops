use async_graphql::futures_util::Stream;
use async_graphql::{async_stream, Context, Object, Subscription};
use serde_json::Value;

use auth::{authenticate, authorize};
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::commons::ws::ExtraWSData;
use domain::pipelines::{PagedPipelines, Pipeline, PipelineStatus, RegisterPipeline};
use persist::db_client::DbClient;

use crate::gql::{get_public_gql_endpoints, shared::paginate};
use crate::services::{jobs, pipelines as service};

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
    async fn pipeline_inserted(&self, ctx: &Context<'_>) -> impl Stream<Item = Pipeline> {
        log::debug!("handling `pipelines::inserted` subscription");
        yield_pipeline(validate_subscription(ctx).await, "create").await
    }

    async fn pipeline_updated(&self, ctx: &Context<'_>) -> impl Stream<Item = Pipeline> {
        log::debug!("handling `pipelines::updated` subscription");
        yield_pipeline(validate_subscription(ctx).await, "update").await
    }

    async fn pipeline_logs(&self, ctx: &Context<'_>) -> impl Stream<Item = String> {
        log::debug!("handling `pipelines::logs` subscription");
        yield_logs(validate_subscription(ctx).await).await
    }
}

async fn yield_pipeline(extras: ExtraWSData, op: &str) -> impl Stream<Item = Pipeline> + '_ {
    let mut receiver = messaging::internal::resubscribe().await;
    async_stream::stream! {
        while let Ok(message) = receiver.recv().await {
            if let Ok(message) = serde_json::from_str::<Value>(&message) {
                let index = message.get("index").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                let operation = message.get("op").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                let item = message.get("item").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                if index == "pipelines" && operation == op {
                    println!("{:?}", item);
                    if let Ok(pipeline) = serde_json::from_str::<Pipeline>(item) {
                        if extras.job_id.is_none() || extras.clone().job_id.unwrap() == pipeline.job_id {
                            yield pipeline;
                        }
                    }
                }
            }
        }
    }
}

async fn yield_logs(extras: ExtraWSData) -> impl Stream<Item = String> + 'static {
    let mut receiver = messaging::internal::resubscribe().await;
    async_stream::stream! {
        while let Ok(message) = receiver.recv().await {
            if let Ok(message) = serde_json::from_str::<Value>(&message) {
                let index = message.get("index").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                let operation = message.get("op").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                let id = message.get("id").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                let entry = message.get("entry").unwrap_or(&Value::Null).as_str().unwrap_or_default();
                if index == "pipelineLogs" && operation == "append"
                    && (extras.pipeline_id.is_none() || extras.clone().pipeline_id.unwrap() == id) {
                    yield entry.to_string();
                }
            }
        }
    }
}

async fn project_id_subscription(db: &DbClient, cred: &Credential, extra: &ExtraWSData) -> String {
    if let Some(job_id) = extra.clone().job_id {
        if let Some(job) = jobs::get_by_id(db, cred, &job_id, &None, &[])
            .await
            .expect("job not found")
        {
            return job.project_id;
        }
    }

    "ALL".to_string()
}

async fn validate_subscription(ctx: &Context<'_>) -> ExtraWSData {
    let db = ctx
        .data::<DbClient>()
        .expect("failed to extract database client");
    let cred = ctx
        .data::<Credential>()
        .expect("failed to extract user credential");
    let username = authenticate(db, cred)
        .await
        .expect("failed to authenticate user");
    let extras = ctx.data::<ExtraWSData>().cloned().unwrap_or_default();
    let project_id = project_id_subscription(db, cred, &extras).await;
    authorize(db, &username, &format!("PROJECTS:READ:{project_id}"))
        .await
        .expect("failed to authorize user");
    authorize(db, &username, &format!("PROJECTS:WRITE:{project_id}"))
        .await
        .expect("failed to authorize user");
    extras
}
