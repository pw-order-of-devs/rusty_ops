use async_graphql::{Context, Object};
use serde_json::Value;

use crate::gql::{get_public_gql_endpoints, shared::paginate};
use crate::services::users as service;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::{
    PagedUserCredentials, PagedUsers, RegisterUser, RegisterUserCredential, UserModel,
};
use domain::commons::search::SearchOptions;
use persist::db_client::DbClient;
use secret::sc_client::ScClient;

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

    #[auth_macro::authenticate(bearer)]
    async fn get_current(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<UserModel>, RustyError> {
        log::debug!("handling `users::getCurrent` request");
        let entry =
            service::get_current(ctx.data::<DbClient>()?, ctx.data::<Credential>()?).await?;
        log::debug!("`users::getCurrent`: found");
        Ok(entry)
    }

    #[auth_macro::authenticate(bearer)]
    async fn get_user_credentials(
        &self,
        ctx: &Context<'_>,
        options: Option<SearchOptions>,
        username: String,
    ) -> async_graphql::Result<PagedUserCredentials, RustyError> {
        log::debug!("handling `users::credentials::get` request");
        let entries = service::get_credentials(
            ctx.data::<DbClient>()?,
            ctx.data::<ScClient>()?,
            ctx.data::<Credential>()?,
            &options,
            &username,
        )
        .await?;
        let (total, page, page_size, entries) = paginate(&entries, None);
        log::debug!("`users::credentials::get`: found {} entries", total);
        Ok(PagedUserCredentials {
            total,
            page,
            page_size,
            entries,
        })
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
        let id = service::create(ctx.data::<DbClient>()?, user).await?;
        log::debug!("`users::register`: registered user with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        username: String,
        old_password: String,
        new_password: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `users::changePassword` request");
        let id = service::change_password(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &username,
            &old_password,
            &new_password,
        )
        .await?;
        log::debug!("`users::changePassword`: changed password for user with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn update_preferences(
        &self,
        ctx: &Context<'_>,
        username: String,
        preferences: String,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `users::updatePreferences` request");
        let id = service::update_preferences(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &username,
            &preferences,
        )
        .await?;
        log::debug!("`users::updatePreferences`: updated preferences for user with id `{id}`");
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn add_credential(
        &self,
        ctx: &Context<'_>,
        username: String,
        credential: RegisterUserCredential,
    ) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `users::addCredential` request");
        let id = service::add_credential(
            ctx.data::<DbClient>()?,
            ctx.data::<ScClient>()?,
            ctx.data::<Credential>()?,
            &username,
            &credential,
        )
        .await?;
        log::debug!(
            "`users::addCredential`: registered new credential `{}` for user with id `{id}`",
            credential.name
        );
        Ok(id)
    }

    #[auth_macro::authenticate(bearer)]
    async fn delete_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> async_graphql::Result<u64, RustyError> {
        log::debug!("handling `users::deleteByUsername` request");
        let result = service::delete_by_username(
            ctx.data::<DbClient>()?,
            ctx.data::<Credential>()?,
            &username,
        )
        .await?;
        log::debug!("`users::deleteByUsername`: deleted user with username `{username}`");
        Ok(result)
    }
}
