use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRecord {
    pub created_at: NaiveDate,
    pub duration: Duration,
    pub description: String,
    pub project: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentTaskRecord {
    pub created_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
    pub description: String,
    pub project: Option<String>,
}

impl CurrentTaskRecord {
    pub fn resume_from(ctr: Self) -> Self {
        Self {
            created_at: Local::now().naive_local(),
            end_at: None,
            description: ctr.description,
            project: ctr.project,
        }
    }
}
