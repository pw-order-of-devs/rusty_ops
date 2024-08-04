#[cfg(test)]
mod mq_client;

#[cfg(test)]
mod messaging {
    use rstest::rstest;

    #[rstest]
    #[should_panic(expected = "Unsupported messaging: ")]
    #[case("", "")]
    #[should_panic(expected = "Unsupported messaging: test")]
    #[case("test", "")]
    #[case("rabbitmq", "RabbitMQ")]
    #[case("rabbit", "RabbitMQ")]
    #[tokio::test]
    async fn init_test(#[case] mq_type: &str, #[case] expected: &str) {
        std::env::set_var("RUSTY_MESSAGING", mq_type);
        let client = messaging::init().await;
        let client_name = format!("{client:?}");
        let client_name = client_name.split('(').collect::<Vec<&str>>()[0];
        std::env::remove_var("RUSTY_MESSAGING");
        assert_eq!(expected, client_name)
    }
}
