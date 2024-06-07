use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::{Image, RunnableImage};
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use rusty_init::ops::roles;

use crate::utils::{create_user, db_connect, USER_ID};

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn create_role_test<I: Image + Default>(
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
    let _ = create_user(&db_client).await;
    let _ = roles::create_role(&db_client, "role_1", "", &[USER_ID]).await;
}