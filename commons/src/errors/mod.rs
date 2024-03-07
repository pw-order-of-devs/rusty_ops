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
    fn from(err: async_graphql::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}

#[cfg(feature = "gloo")]
impl From<gloo_net::Error> for RustyError {
    fn from(err: gloo_net::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for RustyError {
    fn from(err: mongodb::error::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::de::Error> for RustyError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::ser::Error> for RustyError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}

impl From<serde_json::Error> for RustyError {
    fn from(err: serde_json::Error) -> Self {
        log::error!("{err:?}");
        Self {}
    }
}
