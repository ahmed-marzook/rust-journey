use chrono::NaiveDate;

use crate::status::Status;
use crate::{priority::Priority, task::Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoFilter {
    DueDate,
    OverDue,
    Status(Status),
    Priority(Priority),
}

impl TodoFilter {
    pub fn matches(&self, task: &Task, today: NaiveDate) -> bool {
        match self {
            TodoFilter::Status(s) => task.status == *s,
            TodoFilter::Priority(p) => task.priority == Some(*p),
            TodoFilter::DueDate => task.due_date.is_some(),
            TodoFilter::OverDue => {
                // 1. there IS a due date, AND that date is before today.
                //    is_some_and runs the closure only if there's a value:
                //      None             → false (no date, so it can't be overdue)
                //      Some(2026-10-20) → runs |d| d < today → true/false
                let date_passed = task.due_date.is_some_and(|d| d < today);

                // 2. it isn't finished yet. A completed task isn't "overdue",
                //    even if its date has passed.
                let not_done = task.status != Status::Completed;

                date_passed && not_done
            }
        }
    }
}
