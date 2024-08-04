use once_cell::sync::Lazy;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};

use commons::errors::RustyError;

/// Definition of a channel for internal pub/sub usage
#[derive(Debug)]
struct RustyChannel {
    /// sender for pub/sub channel
    pub tx: Mutex<Sender<String>>,
    /// receiver for pub/sub channel
    pub rx: Mutex<Receiver<String>>,
}

/// Static wrapper for global pub/sub channel
static CHANNEL: Lazy<RustyChannel> = Lazy::new(|| {
    let (tx, rx) = broadcast::channel::<String>(100);
    RustyChannel {
        tx: Mutex::new(tx),
        rx: Mutex::new(rx),
    }
});

/// Write message to the global channel
///
/// # Errors
///
/// This function can generate the following errors:
///
/// * `RustyError` - If there was an error during the creation of the item.
pub async fn send(message: &str) -> Result<(), RustyError> {
    let _ = CHANNEL.tx.lock().await.send(message.to_string())?;
    Ok(())
}

/// Get receiver for the global channel messages
pub async fn resubscribe() -> Receiver<String> {
    CHANNEL.rx.lock().await.resubscribe()
}
