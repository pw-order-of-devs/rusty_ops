use rstest::rstest;
use testcontainers::runners::AsyncRunner;
use testcontainers::{Image, ImageExt};
use testcontainers_modules::hashicorp_vault::HashicorpVault;

use crate::utils::sc_connect;

#[rstest]
#[case(HashicorpVault::default(), "vault", 8200)]
#[tokio::test]
async fn put_test<I: Image + Default>(#[case] image: I, #[case] db_type: &str, #[case] port: u16)
where
    I: Image,
{
    std::env::set_var("VAULT_TOKEN", "rusty_token");
    let db = image
        .with_env_var("VAULT_DEV_ROOT_TOKEN_ID", "rusty_token")
        .start()
        .await
        .expect("initializing test container failed");
    let sc_client = sc_connect(&db, db_type, port).await;
    let res = sc_client.put("key", "value").await;
    assert!(res.is_ok());
}

#[rstest]
#[case(HashicorpVault::default(), "vault", 8200)]
#[tokio::test]
async fn get_test<I: Image + Default>(#[case] image: I, #[case] db_type: &str, #[case] port: u16)
where
    I: Image,
{
    std::env::set_var("VAULT_TOKEN", "rusty_token");
    let db = image
        .with_env_var("VAULT_DEV_ROOT_TOKEN_ID", "rusty_token")
        .start()
        .await
        .expect("initializing test container failed");
    let sc_client = sc_connect(&db, db_type, port).await;
    let _ = sc_client.put("key", "value").await;
    let res = sc_client.get("key").await;
    assert!(res.clone().is_ok());
    assert!(res.clone().unwrap().is_some());
    assert_eq!("value", res.unwrap().unwrap());
}
