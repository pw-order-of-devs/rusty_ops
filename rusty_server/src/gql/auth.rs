use async_graphql::{Context, Object};

use auth::token::build_jwt_token;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
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

        let Some(user) = users::get_by_username(ctx.data::<DbClient>()?, username).await? else {
            return Err(RustyError::UnauthenticatedError);
        };
        build_jwt_token(&user, 3600)
    }
}
