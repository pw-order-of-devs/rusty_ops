/// User Credentials
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Credential {
    /// Basic Authentication
    Basic(String, String),
    /// No Authentication
    None,
}

impl std::fmt::Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(user, _) => write!(f, "{user}"),
            Self::None => write!(f, "empty credential"),
        }
    }
}
