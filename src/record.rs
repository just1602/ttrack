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
