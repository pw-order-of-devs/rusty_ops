use chrono::{DateTime, Local};

/// parse rfc3339 date to a common display format
#[must_use]
pub fn parse_date(date: &str) -> String {
    DateTime::parse_from_rfc3339(date)
        .unwrap_or_default()
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
