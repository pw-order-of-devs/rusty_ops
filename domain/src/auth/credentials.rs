/// User Credentials
#[derive(Clone, Debug)]
pub enum Credential {
    /// Basic Authentication
    Basic(String, String),
}
