use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use rusty_init::ops::schema;

use crate::utils::db_connect;

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn purge_db_test<I: Image + Default>(
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
    let _ = schema::purge_db(&db_client).await;
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn execute_sql_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    std::env::set_var("POSTGRESQL_SCRIPTS_PATH", "rusty_init/sql");
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = schema::execute_sql(&db_client, "test").await;
}
