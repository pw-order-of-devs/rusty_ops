use std::time::Duration;

use domain::auth::credentials::get_token_claim_u64;

use crate::api::{auth, JWT_TOKEN};

// schedule a task every x minutes to renew jwt token before it terminates
pub async fn schedule(wait_time: u64) {
    fn is_valid_jwt(token: &str) -> bool {
        token.split('.').count() == 3
    }

    fn calc_wait_time(token: &str) -> u64 {
        let now = chrono::Utc::now().timestamp();
        let now = if now.is_negative() { 0 } else { now as u64 };

        let expiry = get_token_claim_u64(token, "exp");
        let expiry = if expiry < now { now } else { expiry };

        let wait_time = ((expiry - now) as f64 * 0.9).round();
        wait_time.clamp(0., u64::MAX as f64) as u64
    }

    let mut wait_time = wait_time;

    loop {
        log::trace!("waiting for jwt token to be obtained");
        let token = JWT_TOKEN.lock().unwrap().clone();
        if is_valid_jwt(&token) {
            let wait = calc_wait_time(&token);
            wait_time = if wait == 0 { wait_time } else { wait };
            tokio::time::sleep(Duration::from_secs(wait_time)).await;
            break;
        }
        tokio::time::sleep(Duration::from_secs(wait_time)).await;
    }

    loop {
        log::trace!("attempting to renew jwt token");
        let token = auth::renew_token().await.unwrap_or_default();
        if is_valid_jwt(&token) {
            let wait = calc_wait_time(&token);
            wait_time = if wait == 0 { wait_time } else { wait };
            *JWT_TOKEN.lock().unwrap() = token;
            log::trace!("renewed jwt token");
        }
        tokio::time::sleep(Duration::from_secs(wait_time)).await;
    }
}
