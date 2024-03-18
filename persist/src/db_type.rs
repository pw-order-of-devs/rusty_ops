use commons::env::var;

/// The `DbType` enum represents the types of databases supported by the application.
///
/// # Variants
///
/// - `MongoDb`: Represents a `MongoDB` database.
/// - `Redis`: Represents a `Redis` database.
#[derive(Debug)]
pub enum DbType {
    /// A `MongoDb` client for connecting to a `MongoDb` server.
    MongoDb,
    /// A `Redis` client for connecting to a `Redis` server.
    Redis,
}

impl DbType {
    /// Parses the `RUSTY_PERSISTENCE` environment variable and returns the corresponding `DbType` value.
    ///
    /// # Panics
    /// if the `RUSTY_PERSISTENCE` variable is not set or if the value is not supported.
    ///
    /// # Returns
    /// - `DbType::MongoDb` if the `RUSTY_PERSISTENCE` value is `mongodb` or `mongo_db`
    /// - `DbType::Redis` if the `RUSTY_PERSISTENCE` value is `redis`
    #[must_use]
    pub fn parse() -> Self {
        let db_type = var::<String>("RUSTY_PERSISTENCE")
            .expect("RUSTY_PERSISTENCE variable is required")
            .to_lowercase();

        match db_type.as_str() {
            "mongodb" | "mongo_db" => Self::MongoDb,
            "redis" => Self::Redis,
            _ => panic!("Unsupported database: {db_type}"),
        }
    }
}
