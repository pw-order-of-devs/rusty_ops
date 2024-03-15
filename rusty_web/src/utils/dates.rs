use chrono::{DateTime, Local};

/// parse rfc3339 date to a common display format
#[must_use]
pub fn parse_date(date: &Option<String>) -> String {
    date.as_ref().map_or_else(
        || "-".to_string(),
        |date| {
            DateTime::parse_from_rfc3339(date)
                .unwrap_or_default()
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
    )
}
