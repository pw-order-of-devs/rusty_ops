use async_graphql::{Context, EmptySubscription, Object, Result, Schema};
use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::filters::search::SearchFilter;
use domain::jobs::{Job, RegisterJob};
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

fn is_field_requested(ctx: &Context<'_>, field: &str) -> bool {
    ctx.field()
        .selection_set()
        .any(|f| f.name() == field)
}

const PROJECTS_INDEX: &str = "projects";
const JOBS_INDEX: &str = "jobs";

pub struct Query;

#[Object]
impl Query {

    // projects interface
    async fn get_projects(&self, ctx: &Context<'_>, filter: Option<Value>, options: Option<SearchFilter>) -> Result<Vec<Project>, RustyError> {
        log::debug!("handling `get_projects` request");
        let db = get_db_client(ctx)?;
        let mut entries = db.get_all::<Project>(PROJECTS_INDEX, filter, options).await?;

        if is_field_requested(ctx, "jobs") {
            for item in &mut entries {
                let jobs = db.get_all::<Job>(JOBS_INDEX, Some(json!({ "project_id": item.clone().id })), None).await?;
                if !jobs.is_empty() { item.jobs = Some(jobs); }
            };
        }
        log::debug!("`get_projects`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_project_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Project>, RustyError> {
        log::debug!("handling `get_project_by_id` request");
        let db = get_db_client(ctx)?;
        let entry = db.get_by_id::<Project>(PROJECTS_INDEX, &id).await?;
        if entry.is_none() { return Ok(entry) }

        let mut entry = entry.clone().unwrap();
        if is_field_requested(ctx, "jobs") {
            let jobs = db.get_all::<Job>(JOBS_INDEX, Some(json!({ "project_id": entry.clone().id })), None).await?;
            if !jobs.is_empty() { entry.jobs = Some(jobs); }
        }
        Ok(Some(entry))
    }

    // jobs interface
    async fn get_jobs(&self, ctx: &Context<'_>, filter: Option<Value>, options: Option<SearchFilter>) -> Result<Vec<Job>, RustyError> {
        log::debug!("handling `get_jobs` request");
        let entries = get_db_client(ctx)?.get_all(JOBS_INDEX, filter, options).await?;
        log::debug!("`get_jobs`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_job_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Job>, RustyError> {
        log::debug!("handling `get_job_by_id` request");
        get_db_client(ctx)?.get_by_id(JOBS_INDEX, &id).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {

    // projects interface
    async fn register_project(&self, ctx: &Context<'_>, project: RegisterProject) -> Result<String, RustyError> {
        log::info!("handling `register_project` request");
        let project = Project::from(&project);
        get_db_client(ctx)?.create(PROJECTS_INDEX, &project).await
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: String) -> Result<u64, RustyError> {
        log::info!("handling `delete_project` request");
        get_db_client(ctx)?.delete(PROJECTS_INDEX, &id).await
    }

    // jobs interface
    async fn register_job(&self, ctx: &Context<'_>, job: RegisterJob) -> Result<String, RustyError> {
        log::info!("handling `register_job` request");
        let job = Job::from(&job);
        get_db_client(ctx)?.create(JOBS_INDEX, &job).await
    }

    async fn delete_job(&self, ctx: &Context<'_>, id: String) -> Result<u64, RustyError> {
        log::info!("handling `delete_job` request");
        get_db_client(ctx)?.delete(JOBS_INDEX, &id).await
    }
}
