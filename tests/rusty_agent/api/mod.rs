use rusty_agent::api::get_credential;

mod agents;
mod auth;
mod jobs;
mod pipelines;
mod projects;

#[test]
fn get_credentials_test() {
    std::env::set_var("AGENT_USER", "user");
    std::env::set_var("AGENT_PASSWORD", "pass");
    let credential = get_credential();
    assert!(credential.is_ok());
    assert_eq!("dXNlcjpwYXNz", credential.unwrap());
}
