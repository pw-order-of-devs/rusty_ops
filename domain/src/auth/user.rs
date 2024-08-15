use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

use commons::hashing::bcrypt;

use crate::RustyDomainItem;

/// A struct representing a User.
#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserModel {
    /// user id
    pub id: String,
    /// username
    pub username: String,
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
        }
    }
}

impl RustyDomainItem for UserModel {}

impl RustyDomainItem for User {}

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
