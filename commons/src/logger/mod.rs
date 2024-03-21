use std::str::FromStr;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Deserializers, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

use crate::env::{var, var_or_default};

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
    if let Ok(path) = var::<String>("LOG_CONFIG_PATH") {
        match log4rs::init_file(&path, Deserializers::default()) {
            Ok(()) => log::debug!("using logger configuration from: `{path}`"),
            Err(err) => {
                default_logger();
                log::debug!("using default logger configuration: {err}");
            }
        }
    } else {
        default_logger();
        log::debug!("using default logger configuration: LOG_CONFIG_PATH is not defined");
    }
}

fn default_logger() {
    let log_level = var_or_default("LOG_LEVEL", "info".to_string());
    let level_filter = LevelFilter::from_str(&log_level).unwrap_or(LevelFilter::Info);

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("hyper_util", LevelFilter::Off))
        .logger(Logger::builder().build("reqwest", LevelFilter::Off))
        .build(Root::builder().appender("stdout").build(level_filter))
        .unwrap();
    let _ = log4rs::init_config(config);
}
