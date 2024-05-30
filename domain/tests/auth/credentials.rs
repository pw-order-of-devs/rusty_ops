use domain::auth::credentials::{
    get_token_claim_str, get_token_claim_u64, parse_credential, Credential,
};
use rstest::rstest;

const JWT_TOKEN: &str = "eyJhbGciOiJIUzUxMiJ9.eyJpc3MiOiJSdXN0eU9wcyIsInN1YiI6InVzZXIiLCJhdWQiOiJ1c2VyIiwiZXhwIjoxNjE3MDEwNDg4LCJuYmYiOjE2MTcwMTA0ODgsImlhdCI6MTYxNzAxMDQ4OCwianRpIjoiYTQyZDYyN2YtYTEwMC00OWViLTg0MDYtMWZkMWMzMmI2MDMxIn0.";

#[rstest]
#[case(Credential::Basic("test".to_string(), "pass".to_string()), "test")]
#[case(Credential::Bearer(JWT_TOKEN.to_string()), "user")]
#[case(Credential::None, "empty credential")]
fn credential_display_test(#[case] credential: Credential, #[case] expected: &str) {
    assert_eq!(expected, format!("{credential}"))
}

#[rstest]
#[case("eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ0ZXN0In0.", "sub", "test")]
#[case("eyJhbGciOiJIUzUxMiJ9.eyJleHAiOiJ0ZXN0In0.", "exp", "")]
fn get_claim_str_test(#[case] token: &str, #[case] claim: &str, #[case] expected: &str) {
    assert_eq!(expected, get_token_claim_str(token, claim))
}

#[rstest]
#[case("eyJhbGciOiJIUzUxMiJ9.eyJleHAiOjEyM30.", "exp", 123)]
#[case("eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJlcnIifQ.", "sub", 0)]
fn get_claim_numeric_test(#[case] token: &str, #[case] claim: &str, #[case] expected: u64) {
    assert_eq!(expected, get_token_claim_u64(token, claim))
}

#[rstest]
#[case("Basic", "gA==", Credential::None)]
#[case("Basic", "@#$", Credential::None)]
#[case("Basic", "dXNlcnBhc3M=", Credential::None)]
#[case("Basic", "dXNlcjpwYXNz", Credential::Basic("user".to_string(), "pass".to_string()))]
#[case("Bearer", "@#$.@#$.blah", Credential::None)]
#[case("Bearer", "eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ0ZXN0In0", Credential::None)]
#[case("Bearer", "eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJlcnIifQ.blah", Credential::Bearer("eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJlcnIifQ.blah".to_string()))]
fn parse_credential_test(#[case] typ: &str, #[case] value: &str, #[case] credential: Credential) {
    assert_eq!(credential, parse_credential(typ, value))
}
