use std::fmt::{Debug, Display, Formatter};

/// `RustyOps` Error definition
#[derive(Clone, PartialOrd, PartialEq, Eq)]
pub enum RustyError {
    /// AsyncGraphql operation related error
    AsyncGraphqlError {
        /// AsyncGraphql Error message
        message: String,
    },
    /// GlooNet operation related error
    GlooNetError {
        /// GlooNet Error message
        message: String,
    },
    /// MongoDB operation related error
    MongoDBError {
        /// MongoDB Error message
        message: String,
    },
    /// Serde operation related error
    SerializationError {
        /// Serde Error message
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
            RustyError::AsyncGraphqlError { message } => { write!(f, "GraphQL error: {message}") }
            RustyError::GlooNetError { message } => { write!(f, "Gloo error: {message}") }
            RustyError::MongoDBError { message } => { write!(f, "MongoDB error: {message}") }
            RustyError::SerializationError { message } => { write!(f, "Serialization error: {message}") }
        }
    }
}

impl std::error::Error for RustyError {}

impl From<async_graphql::Error> for RustyError {
    fn from(err: async_graphql::Error) -> Self {
        Self::AsyncGraphqlError { message: err.message }
    }
}

#[cfg(feature = "gloo")]
impl From<gloo_net::Error> for RustyError {
    fn from(err: gloo_net::Error) -> Self {
        Self::GlooNetError { message: err.to_string() }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for RustyError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDBError { message: err.kind.to_string() }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::de::Error> for RustyError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        Self::MongoDBError { message: err.to_string() }
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::ser::Error> for RustyError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        Self::MongoDBError { message: err.to_string() }
    }
}

impl From<serde_json::Error> for RustyError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError { message: err.to_string() }
    }
}
