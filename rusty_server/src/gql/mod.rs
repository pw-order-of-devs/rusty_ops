use async_graphql::{Object, Schema};

use persist::db_client::DbClient;

use crate::gql::pipelines::PipelineSubscription;

mod agents;
mod jobs;
mod pipelines;
mod projects;
mod users;

pub type RustySchema = Schema<Query, Mutation, PipelineSubscription>;

pub fn build_schema(database: &DbClient) -> RustySchema {
    Schema::build(Query, Mutation, PipelineSubscription)
        .data(database.clone())
        .finish()
}

pub struct Query;

#[Object]
impl Query {
    // agents interface
    async fn agents(&self) -> agents::AgentsQuery {
        agents::AgentsQuery
    }

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

    // projects interface
    async fn users(&self) -> users::UsersQuery {
        users::UsersQuery
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // agents interface
    async fn agents(&self) -> agents::AgentsMutation {
        agents::AgentsMutation
    }

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

    // projects interface
    async fn users(&self) -> users::UsersMutation {
        users::UsersMutation
    }
}
