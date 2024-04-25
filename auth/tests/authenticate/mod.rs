use testcontainers_modules::{mongo::Mongo, testcontainers::clients::Cli};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::PersistenceBuilder;

#[tokio::test]
async fn basic_auth_test() {
    let cli = Cli::default();
    let db = cli.run(Mongo);
    let connection_string = &format!("mongodb://localhost:{}", db.get_host_port_ipv4(27017));

    let mongo_client = MongoDBClient::from_string(connection_string).await;
    let db_client = DbClient::MongoDb(mongo_client);

    // user does not exist
    let credential = Credential::Basic("user".to_string(), "pass".to_string());
    let authenticated = auth::authenticate(&db_client, &credential).await;
    assert!(authenticated.is_err());
    assert_eq!(RustyError::UserNotFoundError, authenticated.unwrap_err());

    // user exists
    let _ = db_client
        .create(
            "users",
            &User {
                username: "user".to_string(),
                password: "pass".to_string(),
            },
        )
        .await;
    let authenticated = auth::authenticate(&db_client, &credential).await;
    assert!(authenticated.is_ok());
    assert_eq!("user", authenticated.unwrap().username);
}
