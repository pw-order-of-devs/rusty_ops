use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

/// `RustyOps` Error definition
#[derive(Clone, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum RustyError {
    /// Authentication Missing Credential error
    CredentialMissingError,
    /// Authentication Jwt Token Expired error
    JwtTokenExpiredError,
    /// Authentication User Not Found error
    UnauthenticatedError,
    /// Authentication User Not Authorized error
    UnauthorizedError,
    /// Wrong Credential Type error
    WrongCredentialTypeError,

    /// `AsyncGraphql` operation related error
    AsyncGraphqlError(String),
    /// Convert operation related error
    ConvertError(String),
    /// Docker related error
    DockerError(String),
    /// Environment variable error
    EnvVarError(String, String),
    /// Hashing error
    HashingError(String),
    /// IO error
    IoError(String),
    /// JWT error
    JwtError(String),
    /// Messaging error
    MessagingError(String),
    /// `MongoDb` operation related error
    MongoDBError(String),
    /// `PostgresSQL` operation related error
    PostgresSQLError(String),
    /// Redis operation related error
    RedisError(String),
    /// Reqwest operation related error
    RequestError(String),
    /// Serde operation related error
    SerializationError(String),
    /// Tokio operation related error
    TokioError(String),
    /// `serde_valid` operation related error
    ValidationError(String),
    /// Websocket operation related error
    WsError(String),
}

impl Debug for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CredentialMissingError => {
                write!(f, "Auth error: Missing credential")
            }
            Self::JwtTokenExpiredError => {
                write!(f, "Auth error: Jwt token expired")
            }
            Self::UnauthenticatedError => {
                write!(f, "Auth error: Failed to authenticate user")
            }
            Self::UnauthorizedError => {
                write!(f, "Auth error: Failed to authorize user")
            }
            Self::WrongCredentialTypeError => {
                write!(f, "Auth error: Unsupported credential type")
            }

            Self::AsyncGraphqlError(message) => {
                write!(f, "GraphQL error: {message}")
            }
            Self::ConvertError(message) => {
                write!(f, "Convert error: {message}")
            }
            Self::DockerError(message) => {
                write!(f, "Docker error: {message}")
            }
            Self::EnvVarError(key, message) => {
                write!(f, "Env variable error: {key}: {message}")
            }
            Self::HashingError(message) => {
                write!(f, "Hashing error: {message}")
            }
            Self::IoError(message) => {
                write!(f, "IO error: {message}")
            }
            Self::JwtError(message) => {
                write!(f, "JWT error: {message}")
            }
            Self::MessagingError(message) => {
                write!(f, "Messaging error: {message}")
            }
            Self::MongoDBError(message) => {
                write!(f, "MongoDB error: {message}")
            }
            Self::PostgresSQLError(message) => {
                write!(f, "PostgresSQL error: {message}")
            }
            Self::RedisError(message) => {
                write!(f, "Redis error: {message}")
            }
            Self::RequestError(message) => {
                write!(f, "Request error: {message}")
            }
            Self::SerializationError(message) => {
                write!(f, "Serialization error: {message}")
            }
            Self::TokioError(message) => {
                write!(f, "Tokio error: {message}")
            }
            Self::ValidationError(message) => {
                write!(f, "{message}")
            }
            Self::WsError(message) => {
                write!(f, "Websocket error: {message}")
            }
        }
    }
}

impl std::error::Error for RustyError {}

impl From<async_graphql::Error> for RustyError {
    fn from(err: async_graphql::Error) -> Self {
        Self::AsyncGraphqlError(err.message)
    }
}

impl From<reqwest::Error> for RustyError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err.to_string())
    }
}

impl<T: Send> From<tokio::sync::broadcast::error::SendError<T>> for RustyError {
    fn from(err: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Self::MessagingError(err.to_string())
    }
}

#[cfg(feature = "bollard")]
impl From<bollard::errors::Error> for RustyError {
    fn from(err: bollard::errors::Error) -> Self {
        Self::DockerError(err.to_string())
    }
}

#[cfg(feature = "bb8-lapin")]
impl From<bb8_lapin::lapin::Error> for RustyError {
    fn from(err: bb8_lapin::lapin::Error) -> Self {
        Self::MessagingError(err.to_string())
    }
}

#[cfg(feature = "bb8-lapin")]
impl From<bb8_lapin::bb8::RunError<bb8_lapin::lapin::Error>> for RustyError {
    fn from(err: bb8_lapin::bb8::RunError<bb8_lapin::lapin::Error>) -> Self {
        Self::MessagingError(err.to_string())
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for RustyError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDBError(err.kind.to_string())
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::de::Error> for RustyError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        Self::MongoDBError(err.to_string())
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::ser::Error> for RustyError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        Self::MongoDBError(err.to_string())
    }
}

#[cfg(feature = "bb8-postgres")]
impl From<bb8_postgres::tokio_postgres::Error> for RustyError {
    fn from(err: bb8_postgres::tokio_postgres::Error) -> Self {
        Self::PostgresSQLError(err.to_string())
    }
}

#[cfg(feature = "bb8-postgres")]
impl From<bb8_redis::bb8::RunError<bb8_postgres::tokio_postgres::Error>> for RustyError {
    fn from(err: bb8_redis::bb8::RunError<bb8_postgres::tokio_postgres::Error>) -> Self {
        Self::PostgresSQLError(err.to_string())
    }
}

#[cfg(feature = "bb8-redis")]
impl From<bb8_redis::redis::RedisError> for RustyError {
    fn from(err: bb8_redis::redis::RedisError) -> Self {
        Self::RedisError(err.to_string())
    }
}

#[cfg(feature = "bb8-redis")]
impl From<bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>> for RustyError {
    fn from(err: bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>) -> Self {
        Self::RedisError(err.to_string())
    }
}

impl From<serde_json::Error> for RustyError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<serde_valid::validation::Error> for RustyError {
    fn from(err: serde_valid::validation::Error) -> Self {
        Self::ValidationError(err.to_string())
    }
}

impl From<serde_valid::validation::Errors> for RustyError {
    fn from(err: serde_valid::validation::Errors) -> Self {
        Self::ValidationError(err.to_string())
    }
}

impl From<serde_yaml::Error> for RustyError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<base64_url::base64::DecodeError> for RustyError {
    fn from(err: base64_url::base64::DecodeError) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<std::num::TryFromIntError> for RustyError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::ConvertError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for RustyError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::ConvertError(err.to_string())
    }
}

impl From<std::io::Error> for RustyError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for RustyError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::HashingError(err.to_string())
    }
}

impl From<tokio::task::JoinError> for RustyError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TokioError(err.to_string())
    }
}

impl From<jwt::Error> for RustyError {
    fn from(err: jwt::Error) -> Self {
        Self::JwtError(err.to_string())
    }
}

#[cfg(feature = "ws")]
impl From<tokio_tungstenite::tungstenite::Error> for RustyError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WsError(err.to_string())
    }
}
