use rstest::rstest;

use commons::errors::RustyError;

trait VarValue:
std::str::FromStr + Clone +
std::fmt::Display + std::fmt::Debug +
PartialEq + PartialOrd {}

impl VarValue for String {}
impl VarValue for i32 {}

#[rstest]
#[case("TEST_VAR_STR", Some("test".to_string()))]
#[case("TEST_VAR_I32", Some(1234))]
#[case("TEST_VAR_MISSING", None::<String>)]
fn var_test<T: VarValue>(
    #[case] key: &str,
    #[case] value: Option<T>,
) {
    before(key, &value);
    let result = commons::env::var::<T>(key);
    if let Some(value) = value.clone() {
        std::env::remove_var(key);
        assert!(result.is_ok());
        assert_eq!(value, result.unwrap());
    } else {
        assert!(result.is_err());
        assert_eq!(
            RustyError::EnvVarError(
                key.to_string(),
                "environment variable not found".to_string()
            ),
            result.unwrap_err()
        );
    }
}

#[rstest]
#[case("TEST_VAR_STR", Some("test".to_string()))]
fn var_parsing_failing_test<T: VarValue>(
    #[case] key: &str,
    #[case] value: Option<T>,
) {
    before(key, &value);
    let result = commons::env::var::<i32>(key);
    std::env::remove_var(key);
    assert!(result.is_err());
    assert_eq!(
        RustyError::EnvVarError(
            key.to_string(),
            "Failed parsing the result".to_string()
        ),
        result.unwrap_err()
    );
}

#[rstest]
#[case("TEST_VAR_STR", Some("test".to_string()), String::new())]
#[case("TEST_VAR_I32", Some(1234), 0)]
#[case("TEST_VAR_STR_MISSING", None, "default".to_string())]
#[case("TEST_VAR_I32_MISSING", None, 1234)]
fn var_or_default_test<T: VarValue>(
    #[case] key: &str,
    #[case] value: Option<T>,
    #[case] default: T,
) {
    before(key, &value);
    let result = commons::env::var_or_default::<T>(key, default.clone());
    std::env::remove_var(key);
    if let Some(value) = value {
        assert_eq!(value, result);
    } else {
        assert_eq!(default, result);
    }
}

fn before<T: VarValue>(key: &str, value: &Option<T>) {
    if let Some(value) = value {
        std::env::set_var(key, format!("{value}"));
    } else {
        std::env::remove_var(key);
    }
}
