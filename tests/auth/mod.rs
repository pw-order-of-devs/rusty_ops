use rstest::rstest;

use auth::parse_auth_header;
use domain::auth::credentials::Credential;

#[cfg(test)]
mod authenticate;

#[cfg(test)]
mod authorize;

#[cfg(not(tarpaulin_include))]
pub mod utils;

#[rstest]
#[case("", Credential::None)]
#[case("error", Credential::None)]
#[case("something something", Credential::None)]
#[case("Basic c29tZXRoaW5nCg==", Credential::None)]
#[case("Basic dGVzdDp0ZXN0cGFzcw==", Credential::Basic("test".to_string(), "testpass".to_string()))]
fn parse_auth_header_test(#[case] header: &str, #[case] credential: Credential) {
    assert_eq!(credential, parse_auth_header(header))
}
