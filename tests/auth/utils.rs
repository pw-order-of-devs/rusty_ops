use testcontainers::{ContainerAsync, Image};

use commons::errors::RustyError;
use commons::hashing::bcrypt::encode;
use domain::auth::permissions::Permission;
use domain::auth::roles::Role;
use domain::auth::user::User;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::postgre::PostgreSQLClient;
use persist::redis::RedisClient;
use persist::PersistenceBuilder;

pub const USER_ID: &str = "d81e7711-8eed-4cac-9191-d2ec48f36e13";
pub const USER_NAME: &str = "user";
pub const USERS_INDEX: &str = "users";
pub const PERMISSIONS_INDEX: &str = "permissions";
pub const PERMISSION: &str = "RESOURCE:RIGHT";
pub const ROLE_ID: &str = "86ee6a82-cbec-4008-837f-d777ead0477b";
pub const ROLES_INDEX: &str = "roles";

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
                password: encode("pass").unwrap(),
            },
        )
        .await
}

pub async fn create_role(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            ROLES_INDEX,
            &Role {
                id: ROLE_ID.to_string(),
                name: "role".to_string(),
                description: None,
                users: vec![USER_ID.to_string()],
            },
        )
        .await
}

pub async fn create_permission_user(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            PERMISSIONS_INDEX,
            &Permission {
                user_id: Some(USER_ID.to_string()),
                role_id: None,
                resource: "RESOURCE".to_string(),
                right: "RIGHT".to_string(),
            },
        )
        .await
}

pub async fn create_permission_role(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            PERMISSIONS_INDEX,
            &Permission {
                user_id: None,
                role_id: Some(ROLE_ID.to_string()),
                resource: "RESOURCE".to_string(),
                right: "RIGHT".to_string(),
            },
        )
        .await
}
