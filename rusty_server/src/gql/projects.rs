use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::projects::{PagedProjects, ProjectModel, RegisterProject};
use persist::db_client::DbClient;

use crate::gql::{
    get_public_gql_endpoints,
    shared::{paginate, selected_fields},
};
use crate::services::projects as service;

pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    #[auth_macro::authenticate(bearer)]
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedProjects, RustyError> {
        log::debug!("handling `projects::get` request");
        let entries = service::get_all(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &filter,
            &options,
            &selected_fields(ctx),
        )
        .await?;
        let (total, page, page_size, entries) = paginate(&entries, options);
        log::debug!("`projects::get`: found {} entries", total);
        Ok(PagedProjects {
            total,
            page,
            page_size,
            entries,
        })
    }

    #[auth_macro::authenticate(bearer)]
    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        filter: Option<Value>,
    ) -> async_graphql::Result<Option<ProjectModel>, RustyError> {
        log::debug!("handling `projects::getById` request");
        let entry = service::get_by_id(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &id,
            &filter,
            &selected_fields(ctx),
        )
        .await?;
        log::debug!("`projects::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct ProjectsMutation;

#[Object]
impl ProjectsMutation {
    #[auth_macro::authenticate(bearer)]
    async fn register(
        &self,
        ctx: &Context<'_>,
        project: RegisterProject,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `projects::register` request");
        let id =
            service::create(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, project).await?;
        log::debug!("`projects::register`: created project with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::deleteById` request");
        let deleted =
            service::delete_by_id(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, &id).await?;
        log::debug!("`projects::deleteById`: deleted project with id `{id}`");
        Ok(deleted)
    }

    #[auth_macro::authenticate(bearer)]
    async fn delete_all(&self, ctx: &Context<'_>) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `projects::deleteAll` request");
        let deleted = service::delete_all(ctx.data::<DbClient>()?).await?;
        log::debug!("`projects::deleteAll`: deleted {deleted} projects");
        Ok(deleted)
    }
}
