use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// `RustyOps` Error definition
#[derive(Clone, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum RustyError {
    /// AsyncGraphql operation related error
    AsyncGraphqlError {
        /// AsyncGraphql Error message
        message: String,
    },
    /// MongoDB operation related error
    MongoDBError {
        /// MongoDB Error message
        message: String,
    },
    /// Reqwest|Reqwasm operation related error
    RequestError {
        /// Reqwest|Reqwasm Error message
        message: String,
    },
    /// Serde operation related error
    SerializationError {
        /// Serde Error message
        message: String,
    },
    /// Serde_valid operation related error
    ValidationError {
        /// Serde Error message
        message: String,
    },
    /// Websocket operation related error
    WsError {
        /// Websocket Error message
        message: String,
    },
}

impl Debug for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsyncGraphqlError { message } => {
                write!(f, "GraphQL error: {message}")
            }
            Self::RequestError { message } => {
                write!(f, "Request error: {message}")
            }
            Self::MongoDBError { message } => {
                write!(f, "MongoDB error: {message}")
            }
            Self::SerializationError { message } => {
                write!(f, "Serialization error: {message}")
            }
            Self::ValidationError { message } => {
                write!(f, "{message}")
            }
            Self::WsError { message } => {
                write!(f, "Websocket error: {message}")
            }
        }
    }
}

impl std::error::Error for RustyError {}

impl From<async_graphql::Error> for RustyError {
    fn from(err: async_graphql::Error) -> Self {
        Self::AsyncGraphqlError {
            message: err.message,
        }
    }
}

#[cfg(feature = "wasm")]
impl From<reqwasm::Error> for RustyError {
    fn from(err: reqwasm::Error) -> Self {
        Self::RequestError {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for RustyError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError {
            message: err.to_string(),
        }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for RustyError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDBError {
            message: err.kind.to_string(),
        }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::de::Error> for RustyError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        Self::MongoDBError {
            message: err.to_string(),
        }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::ser::Error> for RustyError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        Self::MongoDBError {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for RustyError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError {
            message: err.to_string(),
        }
    }
}

impl From<serde_valid::validation::Error> for RustyError {
    fn from(err: serde_valid::validation::Error) -> Self {
        Self::ValidationError {
            message: err.to_string(),
        }
    }
}

impl From<serde_valid::validation::Errors> for RustyError {
    fn from(err: serde_valid::validation::Errors) -> Self {
        Self::ValidationError {
            message: err.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for RustyError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError {
            message: err.to_string(),
        }
    }
}

impl From<base64_url::base64::DecodeError> for RustyError {
    fn from(err: base64_url::base64::DecodeError) -> Self {
        Self::SerializationError {
            message: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for RustyError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::ValidationError {
            message: err.to_string(),
        }
    }
}

#[cfg(feature = "ws")]
impl From<tokio_tungstenite::tungstenite::Error> for RustyError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WsError {
            message: err.to_string(),
        }
    }
}
