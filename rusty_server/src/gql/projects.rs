use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::filters::search::SearchOptions;
use domain::projects::{Project, RegisterProject};

use crate::gql::get_db_client;
use crate::services::projects as service;

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
        let entries = service::get_all(get_db_client(ctx)?, filter, options).await?;
        log::debug!("`projects::get`: found {} entries", entries.len());
        Ok(entries)
    }

    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Project>, RustyError> {
        log::debug!("handling `projects::getById` request");
        let entry = service::get_by_id(get_db_client(ctx)?, &id).await?;
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
        let id = service::create(get_db_client(ctx)?, project).await?;
        log::debug!("`projects::register`: created project with id `{id}`");
        Ok(id)
    }

    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::deleteById` request");
        let deleted = service::delete_by_id(get_db_client(ctx)?, &id).await?;
        log::debug!("`projects::deleteById`: deleted project with id `{id}`");
        Ok(deleted)
    }
}
