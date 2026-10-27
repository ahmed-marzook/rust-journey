use crate::priority::Priority;
use crate::status::Status;
use chrono::{DateTime, NaiveDate, Utc};

// One row of the tasks table. Changes are made with SQL in task_repo.rs,
// so this is plain data with no methods of its own.
#[derive(Debug)]
pub struct Task {
    // i64 is SQLite's native integer type (and what last_insert_rowid returns).
    pub id: i64,
    pub title: String,
    pub status: Status,
    pub priority: Option<Priority>,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    #[expect(dead_code, reason = "saved for a future 'recently changed' view")]
    pub updated_at: DateTime<Utc>,
}
