use async_graphql::{Context, Object, Result, Schema};

use crate::gql::pipelines::PipelineSubscription;
use persist::{mongo::MongoDBClient, DbType, Persistence};

mod jobs;
mod pipelines;
mod projects;

pub type RustySchema = Schema<Query, Mutation, PipelineSubscription>;

pub fn build_schema(database: impl Persistence + Send + Sync + 'static) -> RustySchema {
    Schema::build(Query, Mutation, PipelineSubscription)
        .data(database)
        .finish()
}

fn get_db_client<'a>(ctx: &Context<'a>) -> Result<&'a impl Persistence> {
    match DbType::parse() {
        DbType::MongoDb => ctx.data::<MongoDBClient>(),
    }
}

pub struct Query;

#[Object]
impl Query {
    // jobs interface
    async fn jobs(&self) -> jobs::JobsQuery {
        jobs::JobsQuery
    }

    // pipelines interface
    async fn pipelines(&self) -> pipelines::PipelinesQuery {
        pipelines::PipelinesQuery
    }

    // projects interface
    async fn projects(&self) -> projects::ProjectsQuery {
        projects::ProjectsQuery
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // jobs interface
    async fn jobs(&self) -> jobs::JobsMutation {
        jobs::JobsMutation
    }

    // pipelines interface
    async fn pipelines(&self) -> pipelines::PipelinesMutation {
        pipelines::PipelinesMutation
    }

    // projects interface
    async fn projects(&self) -> projects::ProjectsMutation {
        projects::ProjectsMutation
    }
}
