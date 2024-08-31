use mockito::ServerGuard;
use serde_json::Value;
use testcontainers::{ContainerAsync, Image};

use commons::errors::RustyError;
use commons::hashing::bcrypt;
use domain::auth::user::User;
use domain::RustyDomainItem;
use messaging::mq_client::MqClient;
use messaging::rabbitmq::RabbitMQClient;
use messaging::MessagingBuilder;
use persist::db_client::DbClient;
use persist::inmemory::InMemoryClient;
use persist::mongo::MongoDBClient;
use persist::postgre::PostgreSQLClient;
use persist::redis::RedisClient;
use persist::PersistenceBuilder;
use secret::sc_client::ScClient;
use secret::vault::VaultClient;
use secret::SecretBuilder;

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
    let connection = if db_type == "internal" {
        ""
    } else {
        let port = db
            .get_host_port_ipv4(port)
            .await
            .expect("failed to obtain container port");
        &format!("{db_type}://{}localhost:{}", auth, port)
    };
    match db_type {
        "internal" => DbClient::InMemory(InMemoryClient::from_string(connection).await),
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

pub async fn mq_connect(mq: &ContainerAsync<impl Image>, mq_type: &str, port: u16) -> MqClient {
    let port = mq
        .get_host_port_ipv4(port)
        .await
        .expect("failed to obtain container port");
    let connection = &format!("amqp://localhost:{}", port);
    match mq_type {
        "rabbit" => MqClient::RabbitMQ(RabbitMQClient::from_string(connection).await),
        _ => panic!("not supported db type"),
    }
}

pub async fn sc_connect(sc: &ContainerAsync<impl Image>, sc_type: &str, port: u16) -> ScClient {
    let port = sc
        .get_host_port_ipv4(port)
        .await
        .expect("failed to obtain container port");
    let connection = &format!("http://localhost:{}", port);
    match sc_type {
        "vault" => ScClient::Vault(VaultClient::from_string(connection).await),
        _ => panic!("not supported sc type"),
    }
}

pub async fn create_user(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            USERS_INDEX,
            &User {
                id: USER_ID.to_string(),
                email: "user@test.org".to_string(),
                username: USER_NAME.to_string(),
                password: bcrypt::encode(USER_PASS)?,
                preferences: Value::Null,
            }
            .to_value()?,
        )
        .await
}
