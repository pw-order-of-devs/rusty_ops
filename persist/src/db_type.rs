use commons::env::var;

/// The `DbType` enum represents the types of databases supported by the application.
///
/// # Variants
///
/// - `InMemory`: Represents an internal database.
/// - `MongoDb`: Represents a `MongoDB` database.
/// - `PostgreSQL`: Represents a `PostgreSQL` database.
/// - `Redis`: Represents a `Redis` database.
#[derive(Debug)]
pub enum DbType {
    /// An internal storage.
    InMemory,
    /// A `MongoDb` client for connecting to a `MongoDb` server.
    MongoDb,
    /// A `PostgreSQL` client for connecting to a `PostgreSQL` server.
    PostgreSQL,
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
    /// - `DbType::InMemory` if the `RUSTY_PERSISTENCE` value is `internal` or `in_memory` or `local`
    /// - `DbType::MongoDb` if the `RUSTY_PERSISTENCE` value is `mongodb` or `mongo_db` or `mongo`
    /// - `DbType::PostgreSQL` if the `RUSTY_PERSISTENCE` value is `postgre` or `postgresql` or `pg`
    /// - `DbType::Redis` if the `RUSTY_PERSISTENCE` value is `redis`
    #[must_use]
    pub fn parse() -> Self {
        let db_type = var::<String>("RUSTY_PERSISTENCE")
            .expect("RUSTY_PERSISTENCE variable is required")
            .to_lowercase();

        match db_type.as_str() {
            "internal" | "in_memory" | "local" => Self::InMemory,
            "mongodb" | "mongo_db" | "mongo" => Self::MongoDb,
            "postgre" | "postgresql" | "pg" => Self::PostgreSQL,
            "redis" => Self::Redis,
            _ => panic!("Unsupported database: {db_type}"),
        }
    }
}
