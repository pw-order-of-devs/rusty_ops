use rstest::rstest;
use serde_json::json;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, Image, RunnableImage};
use testcontainers_modules::{mongo::Mongo, redis::Redis};

use auth::token::build_jwt_token;
use commons::errors::RustyError;
use commons::hashing::bcrypt::encode;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::redis::RedisClient;
use persist::PersistenceBuilder;

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_positive_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
    assert_eq!("user", authenticated.unwrap());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_wrong_credential_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic("user".to_string(), "pass_err".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn basic_auth_no_user_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_positive_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let user_id = create_user(&db_client).await.unwrap();
    let user = db_client
        .get_one::<User>("users", json!({ "id": user_id }))
        .await
        .unwrap()
        .unwrap();
    let token = build_jwt_token(&user, 300);
    assert!(token.is_ok());
    let credential = Credential::Bearer(token.unwrap());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
    assert_eq!("user", authenticated.unwrap());
}

const JWT_TOKEN_EXPIRED: &str = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJSdXN0eU9wcyIsInN1YiI6InVzZXIiLCJhdWQiOiJ1c2VyIiwiZXhwIjoxNjE3MDEwNDg4LCJuYmYiOjE2MTcwMTA0ODgsImlhdCI6MTYxNzAxMDQ4OCwianRpIjoiYTQyZDYyN2YtYTEwMC00OWViLTg0MDYtMWZkMWMzMmI2MDMxIn0.a7EK570ag-KZDSiX-KVAvkcOxwsIVUnU5ho9UrmuOe1TEQC5xgC2EY7LkXyKqOqWgzqE-qMyFS6bq3M6Je3oHQ";

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_expired_token_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let user_id = create_user(&db_client).await.unwrap();
    let _ = db_client
        .get_one::<User>("users", json!({ "id": user_id }))
        .await;
    let credential = Credential::Bearer(JWT_TOKEN_EXPIRED.to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::JwtTokenExpiredError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_invalid_signature_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let user_id = create_user(&db_client).await.unwrap();
    let user = db_client
        .get_one::<User>("users", json!({ "id": user_id }))
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
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn bearer_auth_invalid_token_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::Bearer("blah".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn no_credential_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image).start().await;
    let db_client = db_connect(&db, db_type, port).await;
    let credential = Credential::None;
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
}

async fn db_connect(db: &ContainerAsync<impl Image>, db_type: &str, port: u16) -> DbClient {
    let connection = &format!(
        "{db_type}://localhost:{}",
        db.get_host_port_ipv4(port).await
    );
    match db_type {
        "mongodb" => DbClient::MongoDb(MongoDBClient::from_string(connection).await),
        "redis" => DbClient::Redis(RedisClient::from_string(connection).await),
        _ => panic!("not supported db type"),
    }
}

async fn create_user(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            "users",
            &User {
                id: "d81e7711-8eed-4cac-9191-d2ec48f36e13".to_string(),
                username: "user".to_string(),
                password: encode("pass").unwrap(),
                roles: vec![],
            },
        )
        .await
}
