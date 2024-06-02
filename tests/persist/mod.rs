#[cfg(test)]
mod db_client;

#[cfg(test)]
mod persist {
    use rstest::rstest;

    #[rstest]
    #[should_panic(expected = "Unsupported database: ")]
    #[case("", "")]
    #[should_panic(expected = "Unsupported database: test")]
    #[case("test", "")]
    #[case("mongodb", "MongoDb")]
    #[case("mongo_db", "MongoDb")]
    #[case("mongo", "MongoDb")]
    #[case("pg", "PostgreSql")]
    #[case("postgre", "PostgreSql")]
    #[case("postgresql", "PostgreSql")]
    #[case("redis", "Redis")]
    #[tokio::test]
    async fn init_test(#[case] db_type: &str, #[case] expected: &str) {
        std::env::set_var("RUSTY_PERSISTENCE", db_type);
        let client = persist::init().await;
        let client_name = format!("{client:?}");
        let client_name = client_name.split('(').collect::<Vec<&str>>()[0];
        std::env::remove_var("RUSTY_PERSISTENCE");
        assert_eq!(expected, client_name)
    }
}
