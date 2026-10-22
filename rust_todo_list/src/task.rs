use crate::Priority;
use crate::Status;
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub status: Status,
    pub priority: Option<Priority>,
    pub description: Option<String>,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        matches!(&self.status, Status::Completed)
    }
    pub fn mark_completed(&mut self) {
        self.status = Status::Completed
    }
}
