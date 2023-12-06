use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRecord {
    pub created_at: NaiveDate,
    pub duration: Duration,
    pub description: String,
    pub project: Option<String>,
}

pub fn format_duration(duration: Duration) -> String {
    let mut ret = String::from("");
    let hours = duration.as_secs() / 3600;
    if hours > 0 {
        ret.push_str(&format!("{}h", hours));
    }
    let mins = (duration.as_secs() - hours * 3600) / 60;
    if mins > 0 {
        ret.push_str(&format!("{}m", mins));
    }
    let secs = duration.as_secs() - hours * 3600 - mins * 60;
    if secs > 0 {
        ret.push_str(&format!("{}s", secs));
    }

    ret
}
