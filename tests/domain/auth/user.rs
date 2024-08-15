use rstest::rstest;
use serde_valid::Validate;

use domain::auth::user::{RegisterUser, User};

#[test]
fn from_register_user_test() {
    let username = "test_01";
    let input = RegisterUser::new("user@test.org", username, "password");
    let user = User::from(&input);
    assert_eq!(36, user.id.len());
    assert_eq!(username.to_string(), user.username);
    assert_eq!(60, user.password.len());
}

#[rstest]
#[case(RegisterUser::new("user@test.org", "user", "pass"), true)]
#[case(RegisterUser::new("user@test.org", "user!@#$%^&_-", "pass"), true)]
#[case(RegisterUser::new("user@test.org", "user", ""), false)]
#[case(RegisterUser::new("user@test.org", "", "pass"), false)]
#[case(RegisterUser::new("user@test.org", "[user]", "pass"), false)]
#[case(RegisterUser::new("user@test", "user", "pass"), false)]
#[case(RegisterUser::new("@test.org", "user", "pass"), false)]
#[case(RegisterUser::new("user.test.org", "user", "pass"), false)]
#[case(RegisterUser::new("", "user", "pass"), false)]
fn validate_user_test(#[case] user: RegisterUser, #[case] expected: bool) {
    assert_eq!(expected, user.validate().is_ok())
}
