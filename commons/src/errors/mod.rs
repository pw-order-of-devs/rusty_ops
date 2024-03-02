use std::fmt::{Debug, Display, Formatter};

/// `RustyOps` Error definition
pub struct ROError {}

impl Debug for ROError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error happened")
    }
}

impl Display for ROError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error happened")
    }
}

impl std::error::Error for ROError {}

impl From<async_graphql::Error> for ROError {
    fn from(_: async_graphql::Error) -> Self {
        Self {}
    }
}

impl From<mongodb::error::Error> for ROError {
    fn from(_: mongodb::error::Error) -> Self {
        Self {}
    }
}

impl From<mongodb::bson::de::Error> for ROError {
    fn from(_: mongodb::bson::de::Error) -> Self {
        Self {}
    }
}

impl From<mongodb::bson::ser::Error> for ROError {
    fn from(_: mongodb::bson::ser::Error) -> Self {
        Self {}
    }
}
