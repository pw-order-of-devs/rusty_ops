use domain::agents::{Agent, RegisterAgent};

#[test]
fn from_register_agent_test() {
    let id = uuid::Uuid::new_v4().to_string();
    let input = RegisterAgent { id: id.to_string() };
    let before = chrono::Utc::now().timestamp();
    let agent = Agent::from(&input);
    let after = chrono::Utc::now().timestamp();
    assert_eq!(id, agent.id);
    assert!(before < agent.expiry && agent.expiry > after);
}

#[test]
fn update_expiry_test() {
    let id = uuid::Uuid::new_v4().to_string();
    let input = RegisterAgent { id: id.to_string() };
    let mut agent = Agent::from(&input);
    let before = chrono::Utc::now().timestamp();
    agent.update_expiry();
    let after = chrono::Utc::now().timestamp();
    assert_eq!(id, agent.id);
    assert!(before < agent.expiry && agent.expiry > after);
}
