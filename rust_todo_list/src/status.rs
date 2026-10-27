#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl Status {
    // The exact text stored in the database (and allowed by the CHECK on the
    // status column in db.rs). Kept separate from anything shown to people, so
    // changing how a status is displayed can never break reading saved data.
    // FromStr (above) reads these back.
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Status::Completed => "completed",
            Status::Pending => "pending",
            Status::Skipped => "skipped",
        }
    }
}
