use async_graphql::{Context, Object};
use serde_json::Value;
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::projects::{Project, RegisterProject};
use persist::Persistence;

use crate::gql::get_db_client;

pub(crate) const PROJECTS_INDEX: &str = "projects";

pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<Vec<Project>, RustyError> {
        log::debug!("handling `projects::get` request");
        let entries = get_db_client(ctx)?
            .get_all(PROJECTS_INDEX, filter, options)
            .await?;
        log::debug!("`projects::get`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Project>, RustyError> {
        log::debug!("handling `projects::getById` request");
        get_db_client(ctx)?
            .get_by_id::<Project>(PROJECTS_INDEX, &id)
            .await
    }
}

pub struct ProjectsMutation;

#[Object]
impl ProjectsMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        project: RegisterProject,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `projects::register` request");
        project.validate()?;
        get_db_client(ctx)?
            .create(PROJECTS_INDEX, &Project::from(&project))
            .await
    }

    async fn delete_one(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::delete` request");
        get_db_client(ctx)?.delete(PROJECTS_INDEX, &id).await
    }
}
