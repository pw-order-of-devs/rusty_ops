use async_graphql::{Context, EmptySubscription, Object, Result, Schema};
use serde_json::Value;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::jobs::{Job, RegisterJob};
use domain::pipelines::{Pipeline, RegisterPipeline};
use domain::projects::{Project, RegisterProject};
use persist::{DbType, Persistence, mongo::MongoDBClient};

pub type RustySchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(database: impl Persistence + Send + Sync + 'static) -> RustySchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish()
}

fn get_db_client<'a>(ctx: &Context<'a>) -> Result<&'a impl Persistence> {
    match DbType::parse() {
        DbType::MongoDb => ctx.data::<MongoDBClient>()
    }
}

const PROJECTS_INDEX: &str = "projects";
const JOBS_INDEX: &str = "jobs";
const PIPELINES_INDEX: &str = "pipelines";

pub struct Query;

#[Object]
impl Query {

    // projects interface
    async fn get_projects(&self, ctx: &Context<'_>, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Vec<Project>, RustyError> {
        log::debug!("handling `get_projects` request");
        let entries = get_db_client(ctx)?.get_all::<Project>(PROJECTS_INDEX, filter, options).await?;
        log::debug!("`get_projects`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_project_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Project>, RustyError> {
        log::debug!("handling `get_project_by_id` request");
        get_db_client(ctx)?.get_by_id::<Project>(PROJECTS_INDEX, &id).await
    }

    // jobs interface
    async fn get_jobs(&self, ctx: &Context<'_>, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Vec<Job>, RustyError> {
        log::debug!("handling `get_jobs` request");
        let entries = get_db_client(ctx)?.get_all(JOBS_INDEX, filter, options).await?;
        log::debug!("`get_jobs`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_job_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Job>, RustyError> {
        log::debug!("handling `get_job_by_id` request");
        get_db_client(ctx)?.get_by_id(JOBS_INDEX, &id).await
    }

    // pipelines interface
    async fn get_pipelines(&self, ctx: &Context<'_>, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Vec<Pipeline>, RustyError> {
        log::debug!("handling `get_pipelines` request");
        let entries = get_db_client(ctx)?.get_all(PIPELINES_INDEX, filter, options).await?;
        log::debug!("`get_pipelines`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_pipeline_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Pipeline>, RustyError> {
        log::debug!("handling `get_pipeline_by_id` request");
        get_db_client(ctx)?.get_by_id(PIPELINES_INDEX, &id).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {

    // projects interface
    async fn register_project(&self, ctx: &Context<'_>, project: RegisterProject) -> Result<String, RustyError> {
        log::debug!("handling `register_project` request");
        let project = Project::from(&project);
        get_db_client(ctx)?.create(PROJECTS_INDEX, &project).await
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: String) -> Result<u64, RustyError> {
        log::debug!("handling `delete_project` request");
        get_db_client(ctx)?.delete(PROJECTS_INDEX, &id).await
    }

    // jobs interface
    async fn register_job(&self, ctx: &Context<'_>, job: RegisterJob) -> Result<String, RustyError> {
        log::debug!("handling `register_job` request");
        let job = Job::from(&job);
        get_db_client(ctx)?.create(JOBS_INDEX, &job).await
    }

    async fn delete_job(&self, ctx: &Context<'_>, id: String) -> Result<u64, RustyError> {
        log::debug!("handling `delete_job` request");
        get_db_client(ctx)?.delete(JOBS_INDEX, &id).await
    }

    // pipelines interface
    async fn register_pipeline(&self, ctx: &Context<'_>, pipeline: RegisterPipeline) -> Result<String, RustyError> {
        log::debug!("handling `register_pipeline` request");
        let db = get_db_client(ctx)?;
        let pipelines_count = db.get_all::<Pipeline>(PIPELINES_INDEX, None, None).await?.len() as u64;
        let mut pipeline = Pipeline::from(&pipeline);
        pipeline.number = pipelines_count + 1;
        pipeline.start_date = chrono::Utc::now().to_rfc3339();
        get_db_client(ctx)?.create(PIPELINES_INDEX, &pipeline).await
    }

    async fn delete_pipeline(&self, ctx: &Context<'_>, id: String) -> Result<u64, RustyError> {
        log::debug!("handling `delete_pipeline` request");
        get_db_client(ctx)?.delete(PIPELINES_INDEX, &id).await
    }
}
