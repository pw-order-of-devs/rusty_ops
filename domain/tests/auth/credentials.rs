use domain::auth::credentials::Credential;
use rstest::rstest;

#[rstest]
#[case(Credential::Basic("test".to_string(), "pass".to_string()), "test")]
#[case(Credential::None, "empty credential")]
fn credential_display_test(#[case] credential: Credential, #[case] expected: &str) {
    assert_eq!(expected, format!("{credential}"))
}
