
use std::time::SystemTime;

pub fn format_date(date: SystemTime) -> String {
    date.duration_since(std::time::UNIX_EPOCH)
        .map(|duration| {
            let secs = duration.as_secs();
            let days = (secs / 86400) % 30;
            let months = (secs / 2592000) % 12;
            let years = secs / 31104000;
            format!("{}-{:02}-{:02}", years + 1970, months + 1, days + 1)
        })
        .unwrap_or_else(|_| "Unknown date".to_string())
}
