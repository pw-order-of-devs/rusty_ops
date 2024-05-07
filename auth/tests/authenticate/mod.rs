use commons::errors::RustyError;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, RunnableImage};
use testcontainers_modules::mongo::Mongo;

use commons::hashing::bcrypt::encode;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::PersistenceBuilder;

#[tokio::test]
async fn basic_auth_positive_test() {
    let db = RunnableImage::from(Mongo).start().await;
    let db_client = db_connect(&db).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
    assert_eq!("user", authenticated.unwrap());
}

#[tokio::test]
async fn basic_auth_wrong_credential_test() {
    let db = RunnableImage::from(Mongo).start().await;
    let db_client = db_connect(&db).await;
    let _ = create_user(&db_client).await;
    let credential = Credential::Basic("user".to_string(), "pass_err".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[tokio::test]
async fn basic_auth_no_user_test() {
    let db = RunnableImage::from(Mongo).start().await;
    let db_client = db_connect(&db).await;
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_err());
    assert_eq!(authenticated, Err(RustyError::UnauthenticatedError));
}

#[tokio::test]
async fn no_credential_test() {
    let db = RunnableImage::from(Mongo).start().await;
    let db_client = db_connect(&db).await;
    let credential = Credential::None;
    let authenticated = auth::authenticate(&db_client, &credential).await;
    let _ = db.stop().await;
    assert!(authenticated.is_ok());
}

async fn db_connect(db: &ContainerAsync<Mongo>) -> DbClient {
    let connection_string = &format!("mongodb://localhost:{}", db.get_host_port_ipv4(27017).await);
    let mongo_client = MongoDBClient::from_string(connection_string).await;
    DbClient::MongoDb(mongo_client)
}

async fn create_user(db_client: &DbClient) {
    let _ = db_client
        .create(
            "users",
            &User {
                id: "d81e7711-8eed-4cac-9191-d2ec48f36e13".to_string(),
                username: "user".to_string(),
                password: encode("pass").unwrap(),
                roles: vec![],
            },
        )
        .await;
}
