use testcontainers::runners::AsyncRunner;
use testcontainers::RunnableImage;
use testcontainers_modules::mongo::Mongo;

use commons::hashing::sha512;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::PersistenceBuilder;

#[tokio::test]
async fn basic_auth_test() {
    let db = RunnableImage::from(Mongo).start().await;
    let connection_string = &format!("mongodb://localhost:{}", db.get_host_port_ipv4(27017).await);

    let mongo_client = MongoDBClient::from_string(connection_string).await;
    let db_client = DbClient::MongoDb(mongo_client);

    // user does not exist
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    assert!(authenticated.is_ok());
    assert!(authenticated.clone().unwrap().is_none());

    // user exists
    let _ = db_client
        .create(
            "users",
            &User {
                id: "d81e7711-8eed-4cac-9191-d2ec48f36e13".to_string(),
                username: "user".to_string(),
                password: sha512("pass"),
            },
        )
        .await;
    let authenticated = auth::authenticate(&db_client, &credential).await;
    assert!(authenticated.is_ok());
    assert!(authenticated.clone().unwrap().is_some());
    assert_eq!("user", authenticated.unwrap().unwrap().username);
}
