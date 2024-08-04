use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use rusty_init::versions::v1_0_0::execute;

use crate::utils::db_connect;

#[rstest]
#[case(Redis, "internal", 0)]
#[case(Mongo::default(), "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn version_1_0_0_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) where
    I: Image,
{
    std::env::set_var("POSTGRESQL_SCRIPTS_PATH", "../rusty_init/sql");
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let _ = execute(&db_client).await;
}
