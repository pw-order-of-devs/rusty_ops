#[cfg(test)]
mod sc_client;

#[cfg(test)]
mod secret {
    #[tokio::test]
    async fn init_test() {
        let client = secret::init().await;
        let client_name = format!("{client:?}");
        let client_name = client_name.split('(').collect::<Vec<&str>>()[0];
        assert_eq!("Vault", client_name)
    }
}
