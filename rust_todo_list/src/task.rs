use crate::priority::Priority;
use crate::status::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub priority: Option<Priority>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        matches!(&self.status, Status::Completed)
    }
    pub fn mark_completed(&mut self) {
        self.status = Status::Completed;
        self.updated_at = Utc::now();
    }
}
