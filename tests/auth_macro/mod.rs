use async_graphql::{Context, Object};
use commons::errors::RustyError;

use domain::auth::credentials::Credential;
use persist::db_client::DbClient;

fn get_public_gql_endpoints() -> Vec<String> {
    vec![]
}

struct Query;

#[Object]
impl Query {
    #[auth_macro::authenticate(basic, [])]
    async fn version_basic(&self, ctx: &Context<'_>) -> Result<&str, RustyError> {
        Ok("1.0.0")
    }

    #[auth_macro::authenticate(bearer, [])]
    async fn version_bearer(&self, ctx: &Context<'_>) -> Result<&str, RustyError> {
        Ok("1.0.0")
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use auth::token::build_jwt_token;
    use domain::auth::user::User;
    use serde_json::json;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::redis::Redis;

    use super::*;
    use crate::utils::{create_user, db_connect, USERS_INDEX, USER_ID, USER_NAME, USER_PASS};

    #[tokio::test]
    async fn auth_macro_basic_test() {
        let db = Redis
            .start()
            .await
            .expect("initializing test container failed");
        let db_client = db_connect(&db, "redis", 6379).await;
        let _ = create_user(&db_client).await;

        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(("1".to_string(), "2".to_string(), "3".to_string()))
            .data(db_client)
            .data(Credential::Basic(
                USER_NAME.to_string(),
                USER_PASS.to_string(),
            ))
            .finish();
        let query = "{ versionBasic }";
        let res = schema.execute(query).await;
        assert!(res.errors.is_empty());
    }

    #[tokio::test]
    async fn auth_macro_bearer_test() {
        let db = Redis
            .start()
            .await
            .expect("initializing test container failed");
        let db_client = db_connect(&db, "redis", 6379).await;
        let _ = create_user(&db_client).await;
        let user = db_client
            .get_one::<User>(USERS_INDEX, json!({ "id": { "equals": USER_ID } }))
            .await
            .unwrap()
            .unwrap();
        let token = build_jwt_token(&user, 300);
        assert!(token.is_ok());

        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(("1".to_string(), "2".to_string(), "3".to_string()))
            .data(db_client)
            .data(Credential::Bearer(token.unwrap()))
            .finish();
        let query = "{ versionBearer }";
        let res = schema.execute(query).await;
        assert!(res.errors.is_empty());
    }
}
