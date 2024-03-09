use std::env;
use std::str::FromStr;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

/// Initializes the logging system.
///
/// This function utilizes the `log4rs` crate to configure a logger, which logs
/// to stdout using a pattern.
/// The log level is extracted from the `LOG_LEVEL` environment variable;
/// if it is not set, Info log level is used by default.
///
/// # Panics
///
/// This function will panic if it is unable to build a new Config.
pub fn init() {
    match env::var("LOG_CONFIG_PATH") {
        Ok(path) => match log4rs::init_file(&path, Default::default()) {
            Ok(_) => {
                log::debug!("using logger configuration from: {path}");
            },
            Err(err) => {
                default_logger();
                log::debug!("using default logger configuration: {err}");
            },
        },
        Err(err) => {
            default_logger();
            log::debug!("using default logger configuration: {err}");
        },
    }
}

fn default_logger() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level_filter = LevelFilter::from_str(&log_level).unwrap_or(LevelFilter::Info);

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level_filter))
        .unwrap();
    let _ = log4rs::init_config(config);
}
