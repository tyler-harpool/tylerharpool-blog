use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::time::SystemTime;

pub fn format_date(date: SystemTime) -> String {
    // Convert SystemTime to chrono DateTime
    let datetime: DateTime<Utc> = date.into();

    // Format as YYYY-MM-DD
    datetime.format("%Y-%m-%d").to_string()
}

pub fn parse_date_string(date_str: &str) -> Option<SystemTime> {
    // Parse date string (YYYY-MM-DD format)
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // Convert to a DateTime by assuming midnight UTC
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();

        // Convert to UTC DateTime
        let datetime = Utc.from_utc_datetime(&naive_datetime);

        // Convert to SystemTime
        let system_time = SystemTime::from(datetime);

        return Some(system_time);
    }

    None
}
