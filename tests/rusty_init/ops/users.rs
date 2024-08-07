use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use rusty_init::ops::users;

use crate::utils::db_connect;

#[rstest]
#[case(Redis, "internal", 0)]
#[case(Mongo::default(), "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn create_user_test<I: Image + Default>(
    #[case] image: I,
    #[case] db_type: &str,
    #[case] port: u16,
) {
    let db = image
        .start()
        .await
        .expect("initializing test container failed");
    let db_client = db_connect(&db, db_type, port).await;
    let user_id = users::create_user(&db_client, "admin").await;
    assert!(user_id.is_some());
    assert_eq!(36, user_id.unwrap().len());
}
