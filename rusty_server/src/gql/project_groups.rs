use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::projects::{Group, PagedGroups, RegisterGroup};
use persist::db_client::DbClient;

use crate::gql::{get_public_gql_endpoints, shared::paginate};
use crate::services::project_groups as service;

pub struct ProjectGroupsQuery;

#[Object]
impl ProjectGroupsQuery {
    #[auth_macro::authenticate(bearer, [PROJECTS:READ])]
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedGroups, RustyError> {
        log::debug!("handling `project::groups::get` request");
        let entries = service::get_all(ctx.data::<DbClient>()?, &filter, &options).await?;
        let (total, page, page_size, entries) = paginate(&entries, options);
        log::debug!("`project::groups::get`: found {} entries", total);
        Ok(PagedGroups {
            total,
            page,
            page_size,
            entries,
        })
    }

    #[auth_macro::authenticate(bearer, [PROJECTS:READ])]
    async fn get_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Group>, RustyError> {
        log::debug!("handling `project::groups::getById` request");
        let entry = service::get_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`project::groups::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct ProjectGroupsMutation;

#[Object]
impl ProjectGroupsMutation {
    #[auth_macro::authenticate(bearer, [PROJECTS:WRITE])]
    async fn register(
        &self,
        ctx: &Context<'_>,
        group: RegisterGroup,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `project::groups::register` request");
        let id = service::create(ctx.data::<DbClient>()?, group).await?;
        log::debug!("`project::groups::register`: created project with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer, [PROJECTS:WRITE])]
    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `project::groups::deleteById` request");
        let deleted = service::delete_by_id(ctx.data::<DbClient>()?, &id).await?;
        log::debug!("`project::groups::deleteById`: deleted project with id `{id}`");
        Ok(deleted)
    }

    #[auth_macro::authenticate(bearer, [PROJECTS:WRITE])]
    async fn delete_all(&self, ctx: &Context<'_>) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `project::groups::deleteAll` request");
        let deleted = service::delete_all(ctx.data::<DbClient>()?).await?;
        log::debug!("`project::groups::deleteAll`: deleted {deleted} projects");
        Ok(deleted)
    }
}
