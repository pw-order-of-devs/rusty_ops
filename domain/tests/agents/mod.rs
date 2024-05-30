use domain::agents::{Agent, RegisterAgent};
use domain::RustyDomainItem;

#[test]
fn from_register_agent_test() {
    let id = uuid::Uuid::new_v4().to_string();
    let input = RegisterAgent { id: id.to_string() };
    let before = chrono::Utc::now().timestamp();
    let agent = Agent::from(&input, 300);
    let after = chrono::Utc::now().timestamp();
    assert_eq!(id, agent.get_id());
    assert!(before < agent.expiry && agent.expiry > after);
}

#[test]
fn update_expiry_test() {
    let id = uuid::Uuid::new_v4().to_string();
    let input = RegisterAgent { id: id.to_string() };
    let mut agent = Agent::from(&input, 300);
    let before = chrono::Utc::now().timestamp();
    agent.update_expiry(300);
    let after = chrono::Utc::now().timestamp();
    assert_eq!(id, agent.get_id());
    assert!(before < agent.expiry && agent.expiry > after);
}
