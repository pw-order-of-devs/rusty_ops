use rand::Rng;
use serde_valid::Validate;

use commons::env::var_or_default;
use domain::auth::user::{RegisterUser, User};
use persist::db_client::DbClient;

const USERS_INDEX: &str = "users";

pub async fn create_user(db: &DbClient, user_type: &str) -> String {
    log::info!("creating `{user_type}` user: start");
    let username = var_or_default(
        &format!("{}_USERNAME", user_type.to_uppercase()),
        user_type.to_string(),
    );
    let password = var_or_default(
        &format!("{}_PASSWORD", user_type.to_uppercase()),
        generate_password(12),
    );
    let user = RegisterUser {
        username: username.clone(),
        password: password.clone(),
    };
    user.validate()
        .unwrap_or_else(|_| panic!("error while creating user `{username}`: `validation error`"));
    let user = User::from(&user);
    match db.create(USERS_INDEX, &user).await {
        Ok(id) => {
            log::info!("creating `{user_type}` user: done\n\nusername: {username}\npassword: {password}\nyou should consider changing it!\n");
            id
        }
        Err(err) => panic!("error while creating user `{username}`: `{err}`"),
    }
}

fn generate_password(length: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()"
        .chars()
        .collect::<Vec<char>>();
    (0..length)
        .map(|_| chars[rand::thread_rng().gen_range(0..chars.len())])
        .collect()
}
