use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

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
    /// username
    pub username: String,
    /// password
    pub password: String,
    /// user's role ids
    pub roles: Vec<String>,
}

/// A struct representing the registration of a user.
#[derive(Clone, Debug, InputObject, Serialize, Deserialize, Validate)]
pub struct RegisterUser {
    /// username
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    #[validate(custom(validate_username))]
    pub username: String,
    /// password
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub password: String,
}

fn validate_username(username: &str) -> Result<(), validation::Error> {
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

impl RegisterUser {
    /// constructor
    #[must_use]
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl From<&RegisterUser> for User {
    fn from(value: &RegisterUser) -> Self {
        Self {
            id: Self::generate_id(),
            username: value.clone().username,
            password: value.clone().password,
            roles: vec![],
        }
    }
}

impl RustyDomainItem for UserModel {
    fn get_id(&self) -> String {
        self.clone().id
    }
}

impl RustyDomainItem for User {
    fn get_id(&self) -> String {
        self.clone().id
    }
}

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
