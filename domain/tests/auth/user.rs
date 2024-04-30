use domain::auth::user::{RegisterUser, User};

#[test]
fn from_register_user_test() {
    let username = "test_01";
    let password = "password";
    let input = RegisterUser::new(username, password);
    let user = User::from(&input);
    assert_eq!(36, user.id.len());
    assert_eq!(username.to_string(), user.username);
    assert_eq!(password.to_string(), user.password);
}
