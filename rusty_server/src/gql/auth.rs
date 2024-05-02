use async_graphql::{Context, Object};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use persist::db_client::DbClient;

use crate::gql::get_public_gql_endpoints;

pub struct AuthQuery;

#[Object]
impl AuthQuery {
    #[auth_macro::authenticate]
    async fn login(&self, ctx: &Context<'_>) -> async_graphql::Result<bool, RustyError> {
        log::debug!("handling `auth::login` request");
        // it should produce a jwt token
        Ok(true)
    }
}
