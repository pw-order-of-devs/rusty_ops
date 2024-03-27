use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::projects::{PagedProjects, Project, RegisterProject};
use persist::db_client::DbClient;

use crate::services::projects as service;

pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedProjects, RustyError> {
        log::debug!("handling `projects::get` request");
        let entries = service::get_all_paged(ctx.data::<DbClient>()?, &filter, &options).await?;
        log::debug!("`projects::get`: found {} entries", entries.total);
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Project>, RustyError> {
        log::debug!("handling `projects::getById` request");
        let entry = service::get_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`projects::getById`: found entry by id: `{}`", id);
        Ok(entry)
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
        let id = service::create(ctx.data::<DbClient>()?, project).await?;
        log::debug!("`projects::register`: created project with id `{id}`");
        Ok(id)
    }

    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::deleteById` request");
        let deleted = service::delete_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`projects::deleteById`: deleted project with id `{id}`");
        Ok(deleted)
    }

    async fn delete_all(&self, ctx: &Context<'_>) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::deleteAll` request");
        let deleted = service::delete_all(ctx.data::<DbClient>()?).await?;
        log::debug!("`projects::deleteAll`: deleted {deleted} projects");
        Ok(deleted)
    }
}
