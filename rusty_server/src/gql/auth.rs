use async_graphql::{Context, Object};

use auth::token::build_jwt_token;
use commons::errors::RustyError;
use domain::auth::credentials::{get_token_claim_str, Credential};
use persist::db_client::DbClient;

use crate::gql::get_public_gql_endpoints;
use crate::services::users;

pub struct AuthQuery;

#[Object]
impl AuthQuery {
    #[auth_macro::authenticate_basic]
    async fn login(&self, ctx: &Context<'_>) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `auth::login` request");
        let cred = ctx.data::<Credential>()?;
        let username = match cred {
            Credential::Basic(user, _) => user,
            _ => "",
        };

        build_jwt_token_wrapper(ctx, username).await
    }

    #[auth_macro::authenticate_bearer]
    async fn renew(&self, ctx: &Context<'_>) -> async_graphql::Result<String, RustyError> {
        log::debug!("handling `auth::renew` request");
        let cred = ctx.data::<Credential>()?;
        let username = match cred {
            Credential::Bearer(token) => get_token_claim_str(token, "sub"),
            _ => String::new(),
        };

        build_jwt_token_wrapper(ctx, &username).await
    }
}

async fn build_jwt_token_wrapper(
    ctx: &Context<'_>,
    username: &str,
) -> async_graphql::Result<String, RustyError> {
    let Some(user) = users::get_by_username(ctx.data::<DbClient>()?, username).await? else {
        return Err(RustyError::UnauthenticatedError);
    };
    build_jwt_token(&user, 3600)
}
