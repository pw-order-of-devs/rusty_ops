use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_valid::{validation, Validate};

use commons::hashing::bcrypt;

use crate::RustyDomainItem;

/// A struct representing a User.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserModel {
    /// user id
    pub id: String,
    /// user email address
    pub email: String,
    /// username
    pub username: String,
    /// preferences
    pub preferences: Value,
}

/// A struct representing a User.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct User {
    /// user id
    pub id: String,
    /// user email address
    pub email: String,
    /// username
    pub username: String,
    /// password
    pub password: String,
    /// preferences
    #[serde(default = "default_preferences")]
    pub preferences: Value,
}

/// A struct representing the registration of a user.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterUser {
    /// email
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    #[validate(custom(validate_email))]
    pub email: String,
    /// username
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    #[validate(custom(validate_username))]
    pub username: String,
    /// password
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    #[validate(custom(validate_password))]
    pub password: String,
}

/// An enum representing a credential source (repository).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
pub enum CredSource {
    /// credential referencing GitHub
    GitHub,
}

/// A struct representing a User Credential.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserCredential {
    /// credential id
    pub id: String,
    /// credential name
    pub name: String,
    /// credential source
    pub source: CredSource,
    /// credential token value
    pub token: String,
    /// credential user id
    pub user_id: String,
}

/// A struct representing a User Credential Model.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserCredentialModel {
    /// credential id
    pub id: String,
    /// credential name
    pub name: String,
    /// credential source
    pub source: CredSource,
    /// credential user id
    pub user_id: String,
}

/// A struct representing a User Credential.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterUserCredential {
    /// credential name
    #[validate(min_length = 1)]
    #[validate(max_length = 128)]
    pub name: String,
    /// credential source
    pub source: CredSource,
    /// credential token value
    #[validate(min_length = 1)]
    pub token: String,
}

fn default_preferences() -> Value {
    Value::Object(Map::new())
}

fn validate_email(email: &str) -> Result<(), validation::Error> {
    let pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    regex::Regex::new(pattern).map_or_else(
        |_| Err(validation::Error::Custom("invalid email regex".to_owned())),
        |reg| {
            if reg.is_match(email) {
                Ok(())
            } else {
                Err(validation::Error::Custom(
                    "invalid email address".to_owned(),
                ))
            }
        },
    )
}

fn validate_username(username: &str) -> Result<(), validation::Error> {
    if username == "SYSTEM" {
        return Err(validation::Error::Custom("restricted username".to_owned()));
    };

    let allowed = ['!', '@', '#', '$', '%', '^', '&', '_', '-'];
    if username
        .chars()
        .all(|c| c.is_alphanumeric() || allowed.contains(&c))
    {
        Ok(())
    } else {
        Err(validation::Error::Custom(
            "username contains disallowed characters".to_owned(),
        ))
    }
}

fn validate_password(password: &str) -> Result<(), validation::Error> {
    if bcrypt::encode(password).is_ok() {
        Ok(())
    } else {
        Err(validation::Error::Custom(
            "password hashing failed".to_owned(),
        ))
    }
}

impl RegisterUser {
    /// constructor
    #[must_use]
    pub fn new(email: &str, username: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl From<&RegisterUser> for User {
    fn from(value: &RegisterUser) -> Self {
        Self {
            id: Self::generate_id(),
            email: value.clone().email,
            username: value.clone().username,
            password: bcrypt::encode(&value.password).unwrap_or_default(),
            preferences: Value::Object(Map::new()),
        }
    }
}

impl From<&User> for UserModel {
    fn from(value: &User) -> Self {
        Self {
            id: value.clone().id,
            email: value.clone().email,
            username: value.clone().username,
            preferences: value.clone().preferences,
        }
    }
}

impl RustyDomainItem for UserModel {}

impl RustyDomainItem for User {}

impl RustyDomainItem for UserCredential {}

impl RustyDomainItem for UserCredentialModel {}

/// A struct representing a paged result Users.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedUsers {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<UserModel>,
}

/// A struct representing a paged result Users.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct PagedUserCredentials {
    /// total amount of entries found
    pub total: usize,
    /// current page
    pub page: usize,
    /// size of a page
    pub page_size: usize,
    /// data returned by query
    pub entries: Vec<UserCredentialModel>,
}
