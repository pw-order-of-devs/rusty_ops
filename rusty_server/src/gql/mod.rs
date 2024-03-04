use async_graphql::{Context, EmptySubscription, Object, Result, Schema};

use commons::errors::RustyError;
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

const PROJECTS_INDEX: &str = "projects";
const JOBS_INDEX: &str = "jobs";

pub struct Query;

#[Object]
impl Query {

    // projects interface
    async fn get_projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>, RustyError> {
        log::info!("handling `get_projects` request");
        let db = get_db_client(ctx)?;
        db.get_all::<Project>(PROJECTS_INDEX).await
    }

    async fn get_project_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Project>, RustyError> {
        log::info!("handling `get_project_by_id` request");
        get_db_client(ctx)?.get_by_id(PROJECTS_INDEX, &id).await
    }

    // jobs interface
    async fn get_jobs(&self, ctx: &Context<'_>) -> Result<Vec<Job>, RustyError> {
        log::info!("handling `get_jobs` request");
        get_db_client(ctx)?.get_all(JOBS_INDEX).await
    }

    async fn get_job_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Job>, RustyError> {
        log::info!("handling `get_job_by_id` request");
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
