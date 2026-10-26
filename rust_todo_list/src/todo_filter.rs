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

// Lets you write "overdue".parse::<TodoFilter>(), the same way
// Priority and Status parse themselves.
impl std::str::FromStr for TodoFilter {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            // Keywords that are only filters (not a status or a priority)
            "due" => Ok(TodoFilter::DueDate),
            "overdue" => Ok(TodoFilter::OverDue),

            // Anything else: try it as a status first...
            other => other
                .parse::<Status>()
                .map(TodoFilter::Status)
                // ...if that failed, try it as a priority...
                .or_else(|_| other.parse::<Priority>().map(TodoFilter::Priority))
                // ...if both failed, one clear message listing every option.
                .map_err(|_| {
                    format!(
                        "Unknown filter '{other}'. Use: due, overdue, \
                         pending, done, skipped, high, medium, low"
                    )
                }),
        }
    }
}
