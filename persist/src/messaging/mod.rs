use once_cell::sync::Lazy;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

/// Definition of a channel for internal pub/sub usage
#[derive(Debug)]
pub struct RustyChannel {
    /// sender for pub/sub channel
    pub tx: Mutex<Sender<String>>,
    /// receiver for pub/sub channel
    pub rx: Mutex<Receiver<String>>,
}

/// Static wrapper for global pub/sub channel
pub static CHANNEL: Lazy<RustyChannel> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<String>(100);
    RustyChannel {
        tx: Mutex::new(tx),
        rx: Mutex::new(rx),
    }
});
