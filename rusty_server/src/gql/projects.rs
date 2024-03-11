use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::projects::{Project, RegisterProject};
use persist::Persistence;

use crate::gql::get_db_client;

const PROJECTS_INDEX: &str = "projects";

pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    // projects interface
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Project>, RustyError> {
        log::debug!("handling `get_projects` request");
        let entries = get_db_client(ctx)?
            .get_all(PROJECTS_INDEX, filter, options)
            .await?;
        log::debug!("`get_projects`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Project>, RustyError> {
        log::debug!("handling `get_project_by_id` request");
        get_db_client(ctx)?
            .get_by_id::<Project>(PROJECTS_INDEX, &id)
            .await
    }
}

pub struct ProjectsMutation;

#[Object]
impl ProjectsMutation {
    // projects interface
    async fn register(
        &self,
        ctx: &Context<'_>,
        project: RegisterProject,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `register_project` request");
        let project = Project::from(&project);
        get_db_client(ctx)?.create(PROJECTS_INDEX, &project).await
    }

    async fn delete_one(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `delete_project` request");
        get_db_client(ctx)?.delete(PROJECTS_INDEX, &id).await
    }
}
