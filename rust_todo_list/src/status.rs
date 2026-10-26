use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Completed,
    Pending,
    Skipped,
}

impl std::str::FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "done" | "completed" => Ok(Status::Completed),
            "pending" => Ok(Status::Pending),
            "skipped" => Ok(Status::Skipped),
            other => Err(format!(
                "Unknown status '{other}'. Use: completed (or done), pending, or skipped"
            )),
        }
    }
}
