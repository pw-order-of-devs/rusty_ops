use async_graphql::{Object, Schema};

use persist::db_client::DbClient;

use crate::gql::pipelines::PipelineSubscription;

mod jobs;
mod pipelines;
mod projects;

pub type RustySchema = Schema<Query, Mutation, PipelineSubscription>;

pub fn build_schema(database: &DbClient) -> RustySchema {
    Schema::build(Query, Mutation, PipelineSubscription)
        .data(database.clone())
        .finish()
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
