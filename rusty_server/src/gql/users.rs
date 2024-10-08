use async_graphql::{Context, Object};
use serde_json::Value;

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::{PagedUsers, RegisterUser, UserModel};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;

use crate::gql::{get_public_gql_endpoints, shared::paginate};
use crate::services::users as service;

pub struct UsersQuery;

#[Object]
impl UsersQuery {
    #[auth_macro::authenticate(bearer)]
    async fn get(
        &self,
        ctx: &Context<'_>,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> async_graphql::Result<PagedUsers, RustyError> {
        log::debug!("handling `users::get` request");
        let entries = service::get_all(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &filter,
            &options,
        )
        .await?;
        let (total, page, page_size, entries) = paginate(&entries, options);
        log::debug!("`users::get`: found {} entries", total);
        Ok(PagedUsers {
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
    ) -> async_graphql::Result<Option<UserModel>, RustyError> {
        log::debug!("handling `users::getById` request");
        let entry =
            service::get_by_id(ctx.data::<DbClient>()?, ctx.data::<Credential>()?, &id).await?;
        log::debug!("`users::getById`: found entry by id: `{}`", id);
        Ok(entry)
    }
}

pub struct UsersMutation;

#[Object]
impl UsersMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        user: RegisterUser,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `users::register` request");
        let id = service::create(ctx.data::<DbClient>()?, &Credential::System, user).await?;
        log::debug!("`users::register`: registered user with id `{id}`");
        Ok(id)
    }
}
