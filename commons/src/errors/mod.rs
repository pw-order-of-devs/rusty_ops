use std::fmt::{Debug, Display, Formatter};

/// `RustyOps` Error definition
#[derive(Clone)]
pub struct RustyError {}

impl Debug for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error happened")
    }
}

impl Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error happened")
    }
}

impl std::error::Error for RustyError {}

impl From<async_graphql::Error> for RustyError {
    fn from(_: async_graphql::Error) -> Self {
        Self {}
    }
}

#[cfg(feature = "gloo")]
impl From<gloo_net::Error> for RustyError {
    fn from(_: gloo_net::Error) -> Self {
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for RustyError {
    fn from(_: mongodb::error::Error) -> Self {
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::de::Error> for RustyError {
    fn from(_: mongodb::bson::de::Error) -> Self {
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::ser::Error> for RustyError {
    fn from(_: mongodb::bson::ser::Error) -> Self {
        Self {}
    }
}

impl From<serde_json::Error> for RustyError {
    fn from(_: serde_json::Error) -> Self {
        Self {}
    }
}
