use mockito::ServerGuard;
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

pub async fn mockito_start_server() -> ServerGuard {
    let server = mockito::Server::new_async().await;
    let host_port = server.host_with_port();
    let host_port = host_port.split(":").collect::<Vec<&str>>();
    std::env::set_var("SERVER_PROTOCOL", "http");
    std::env::set_var("SERVER_HOST", host_port[0]);
    std::env::set_var("SERVER_PORT", host_port[1]);
    server
}
