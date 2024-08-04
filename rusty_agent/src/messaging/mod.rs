use commons::errors::RustyError;
use messaging::mq_client::MqClient;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

static MESSAGING: OnceCell<Mutex<MqClient>> = OnceCell::new();

pub async fn get_messaging() -> Result<&'static Mutex<MqClient>, RustyError> {
    match MESSAGING.get() {
        Some(client) => Ok(client),
        None => {
            let client = messaging::init().await;
            let _ = MESSAGING.set(Mutex::new(client)).map_err(|_| {
                RustyError::MessagingError("Failed to initialize messaging client".to_string())
            });
            Ok(MESSAGING.get().unwrap())
        }
    }
}
