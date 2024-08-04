use domain::agents::{Agent, RegisterAgent};

#[test]
fn from_register_agent_test() {
    let input = RegisterAgent {
        id: uuid::Uuid::new_v4().to_string(),
    };
    let before = chrono::Utc::now().timestamp();
    let agent = Agent::from(&input, 300);
    let after = chrono::Utc::now().timestamp();
    assert!(before < agent.expiry && agent.expiry > after);
}

#[test]
fn update_expiry_test() {
    let input = RegisterAgent {
        id: uuid::Uuid::new_v4().to_string(),
    };
    let mut agent = Agent::from(&input, 300);
    let before = chrono::Utc::now().timestamp();
    agent.update_expiry(300);
    let after = chrono::Utc::now().timestamp();
    assert!(before < agent.expiry && agent.expiry > after);
}
