use rstest::rstest;
use serde_json::json;
use std::str::FromStr;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, Image, RunnableImage};
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::projects::Project;
use persist::db_client::DbClient;
use persist::mongo::MongoDBClient;
use persist::postgre::PostgreSQLClient;
use persist::redis::RedisClient;
use persist::PersistenceBuilder;

const PROJECTS_INDEX: &str = "projects";

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn get_all_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_project(&db_client, "project_1").await;
    let _ = create_project(&db_client, "project_2").await;
    let _ = create_project(&db_client, "project_2").await;
    let _ = create_project(&db_client, "project_3").await;

    let results = db_client
        .get_all::<Project>(PROJECTS_INDEX, &None, &None, false)
        .await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(4, results.len());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn get_all_paged_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_project(&db_client, "project_1").await;
    let _ = create_project(&db_client, "project_2").await;
    let _ = create_project(&db_client, "project_2").await;
    let _ = create_project(&db_client, "project_3").await;

    let results = db_client
        .get_all::<Project>(
            PROJECTS_INDEX,
            &None,
            &Some(SearchOptions {
                page_number: Some(2),
                page_size: Some(2),
                sort_field: None,
                sort_mode: None,
            }),
            true,
        )
        .await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(2, results.len());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn create_test<I: Image + Default>(#[case] image: I, #[case] db_type: &str, #[case] port: u16)
where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let result = create_project(&db_client, "project_1").await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(uuid::Uuid::from_str(&result).is_ok());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn update_test<I: Image + Default>(#[case] image: I, #[case] db_type: &str, #[case] port: u16)
where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let result = create_project(&db_client, "project_1").await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(uuid::Uuid::from_str(&result).is_ok());

    let updated = db_client
        .update(
            PROJECTS_INDEX,
            &result,
            &Project {
                id: result.clone(),
                name: "project_1_u".to_string(),
                url: Some("url://project_1.ext".to_string()),
                group_id: None,
            },
        )
        .await;
    assert!(updated.is_ok());
    let updated = updated.unwrap();
    assert!(uuid::Uuid::from_str(&updated).is_ok());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn delete_one_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let result = create_project(&db_client, "project_1").await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(uuid::Uuid::from_str(&result).is_ok());

    let deleted = db_client
        .delete_one::<Project>(PROJECTS_INDEX, json!({ "id": result }))
        .await;
    assert!(deleted.is_ok());
    let deleted = deleted.unwrap();
    assert_eq!(1, deleted);
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn delete_all_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = create_project(&db_client, "project_1").await;
    let _ = create_project(&db_client, "project_2").await;
    let _ = create_project(&db_client, "project_3").await;
    let _ = create_project(&db_client, "project_4").await;

    let deleted = db_client.delete_all(PROJECTS_INDEX).await;
    assert!(deleted.is_ok());
    let deleted = deleted.unwrap();
    assert_eq!(4, deleted);
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn purge_test<I: Image + Default>(#[case] image: I, #[case] db_type: &str, #[case] port: u16)
where
    <I as Image>::Args: Default,
{
    let db = RunnableImage::from(image)
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;

    let result = db_client.purge().await;
    assert!(result.is_ok());
}

async fn db_connect(db: &ContainerAsync<impl Image>, db_type: &str, port: u16) -> DbClient {
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

async fn create_project(db_client: &DbClient, name: &str) -> Result<String, RustyError> {
    db_client
        .create(
            PROJECTS_INDEX,
            &Project {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                url: Some(format!("url://{name}.ext")),
                group_id: None,
            },
        )
        .await
}
