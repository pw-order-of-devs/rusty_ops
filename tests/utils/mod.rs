use mockito::ServerGuard;
use testcontainers::{ContainerAsync, Image};

use commons::errors::RustyError;
use commons::hashing::bcrypt;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::postgre::PostgreSQLClient;
use persist::redis::RedisClient;
use persist::PersistenceBuilder;

pub const USER_ID: &str = "d81e7711-8eed-4cac-9191-d2ec48f36e13";
pub const USER_NAME: &str = "user";
pub const USER_PASS: &str = "pass";
pub const USERS_INDEX: &str = "users";

pub async fn mockito_start_server() -> ServerGuard {
    let server = mockito::Server::new_async().await;
    let host_port = server.host_with_port();
    let host_port = host_port.split(":").collect::<Vec<&str>>();
    std::env::set_var("SERVER_PROTOCOL", "http");
    std::env::set_var("SERVER_HOST", host_port[0]);
    std::env::set_var("SERVER_PORT", host_port[1]);
    server
}

pub async fn db_connect(db: &ContainerAsync<impl Image>, db_type: &str, port: u16) -> DbClient {
    let auth = if db_type == "postgres" {
        "postgres:postgres@"
    } else {
        ""
    };
    let connection = &format!(
        "{db_type}://{}localhost:{}",
        auth,
        db.get_host_port_ipv4(port)
            .await
            .expect("failed to obtain container port")
    );
    match db_type {
        "mongodb" => DbClient::MongoDb(MongoDBClient::from_string(connection).await),
        "postgres" => {
            std::env::set_var("POSTGRESQL_SCHEMA", "rusty");
            let client = PostgreSQLClient::from_string(connection).await;
            let _ = client.execute_sql_dir("../rusty_init/sql").await;
            DbClient::PostgreSql(client)
        }
        "redis" => DbClient::Redis(RedisClient::from_string(connection).await),
        _ => panic!("not supported db type"),
    }
}

pub async fn create_user(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            USERS_INDEX,
            &User {
                id: USER_ID.to_string(),
                username: USER_NAME.to_string(),
                password: bcrypt::encode(USER_PASS).unwrap(),
            },
        )
        .await
}
