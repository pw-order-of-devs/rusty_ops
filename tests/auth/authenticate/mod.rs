use rstest::rstest;
use serde_json::json;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use auth::token::build_jwt_token;
use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::User;

use crate::utils::{create_user, db_connect, USERS_INDEX, USER_ID, USER_NAME};

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_positive_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic(USER_NAME.to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
    assert_eq!("user", authenticated.unwrap());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_wrong_credential_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic(USER_NAME.to_string(), "pass_err".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_no_user_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::Basic(USER_NAME.to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_positive_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await.unwrap();
    let user = db_client
        .get_one::<User>(USERS_INDEX, json!({ "id": { "equals": USER_ID } }))
        .await
        .unwrap()
        .unwrap();
    let token = build_jwt_token(&user, 300);
    assert!(token.is_ok());
    let credential = Credential::Bearer(token.unwrap());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
    assert_eq!(USER_NAME, authenticated.unwrap());
}

const JWT_TOKEN_EXPIRED: &str = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJSdXN0eU9wcyIsInN1YiI6InVzZXIiLCJhdWQiOiJ1c2VyIiwiZXhwIjoxNjE3MDEwNDg4LCJuYmYiOjE2MTcwMTA0ODgsImlhdCI6MTYxNzAxMDQ4OCwianRpIjoiYTQyZDYyN2YtYTEwMC00OWViLTg0MDYtMWZkMWMzMmI2MDMxIn0.a7EK570ag-KZDSiX-KVAvkcOxwsIVUnU5ho9UrmuOe1TEQC5xgC2EY7LkXyKqOqWgzqE-qMyFS6bq3M6Je3oHQ";

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_expired_token_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await.unwrap();
    let _ = db_client
        .get_one::<User>(USERS_INDEX, json!({ "id": { "equals": USER_ID } }))
        .await;
    let credential = Credential::Bearer(JWT_TOKEN_EXPIRED.to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::JwtTokenExpiredError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_invalid_signature_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await.unwrap();
    let user = db_client
        .get_one::<User>(USERS_INDEX, json!({ "id": { "equals": USER_ID } }))
        .await
        .unwrap()
        .unwrap();
    let token = build_jwt_token(&user, 300);
    assert!(token.is_ok());
    let token = token
        .unwrap()
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".")
        + ".err";
    let credential = Credential::Bearer(token);
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_invalid_token_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::Bearer("blah".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn no_credential_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::None;
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
}
