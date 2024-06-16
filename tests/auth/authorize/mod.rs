use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::Image;
use testcontainers_modules::{mongo::Mongo, postgres::Postgres, redis::Redis};

use commons::errors::RustyError;

use crate::auth::utils::{create_permission_role, create_permission_user, create_role, PERMISSION};
use crate::utils::{create_user, db_connect, USER_NAME};

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn authorize_no_user_test<I: Image + Default>(
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
    let authorized = auth::authorize(&db_client, "dummy", "dummy:dummy").await;
    let _ = db.stop().await;
    assert!(authorized.is_err());
    assert_eq!(
        RustyError::RequestError("User was not found".to_string()),
        authorized.unwrap_err()
    );
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn authorize_no_permissions_test<I: Image + Default>(
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
    let _ = create_user(&db_client).await.unwrap();
    let authorized = auth::authorize(&db_client, USER_NAME, "dummy:dummy").await;
    let _ = db.stop().await;
    assert!(authorized.is_err());
    assert_eq!(RustyError::UnauthenticatedError, authorized.unwrap_err());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn authorize_user_positive_test<I: Image + Default>(
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
    let _ = create_user(&db_client).await.unwrap();
    let _ = create_permission_user(&db_client).await.unwrap();
    let authorized = auth::authorize(&db_client, USER_NAME, PERMISSION).await;
    let _ = db.stop().await;
    assert!(authorized.is_ok());
}

#[rstest]
#[case(Mongo, "mongodb", 27017)]
#[case(Postgres::default(), "postgres", 5432)]
#[case(Redis, "redis", 6379)]
#[tokio::test]
async fn authorize_role_positive_test<I: Image + Default>(
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
    let _ = create_user(&db_client).await.unwrap();
    let _ = create_role(&db_client).await.unwrap();
    let _ = create_permission_role(&db_client).await.unwrap();
    let authorized = auth::authorize(&db_client, USER_NAME, PERMISSION).await;
    let _ = db.stop().await;
    assert!(authorized.is_ok());
}
