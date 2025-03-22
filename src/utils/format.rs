use chrono::{DateTime, Utc};


// Option 1: Update function to accept DateTime<Utc> directly
pub fn format_date(date: DateTime<Utc>) -> String {
    // Format the date nicely (e.g., "January 15, 2023")
    date.format("%B %d, %Y").to_string()
}
