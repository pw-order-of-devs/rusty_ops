use async_graphql::{Context, EmptySubscription, Object, Result, Schema};

use commons::errors::ROError;
use domain::jobs::{Job, RegisterJobModel};
use persist::{DbType, Persistence, mongo::MongoDBClient, };

pub type ROSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(database: impl Persistence + Send + Sync + 'static) -> ROSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish()
}

fn get_db_client<'a>(ctx: &Context<'a>) -> Result<&'a impl Persistence> {
    match DbType::parse() {
        DbType::MongoDb => ctx.data::<MongoDBClient>()
    }
}

const JOBS_INDEX: &str = "jobs";

pub struct Query;

#[Object]
impl Query {

    // jobs interface
    async fn get_jobs(&self, ctx: &Context<'_>) -> Result<Vec<Job>, ROError> {
        get_db_client(ctx)?.get_all(JOBS_INDEX).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {

    // jobs interface
    async fn register_job(&self, ctx: &Context<'_>, job: RegisterJobModel) -> Result<String, ROError> {
        get_db_client(ctx)?.create(JOBS_INDEX, &job).await
    }

    async fn delete_jobs(&self, ctx: &Context<'_>, job: RegisterJobModel) -> Result<u64, ROError> {
        get_db_client(ctx)?.delete(JOBS_INDEX, &job).await
    }
}
