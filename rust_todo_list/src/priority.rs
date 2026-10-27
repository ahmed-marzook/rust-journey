#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl std::str::FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" | "h" => Ok(Priority::High),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "low" | "l" => Ok(Priority::Low),
            // ... your other arms here ...
            other => Err(format!(
                "Unknown priority '{other}'. Use: high (h), medium (m), low (l) or none"
            )),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        f.pad(text) // ← use pad, not write!: see Part 3
    }
}

impl Priority {
    // The exact text stored in the database (and allowed by the CHECK on the
    // priority column in db.rs). Not the same as Display: that's for people
    // and may change; this must stay fixed. FromStr (above) reads these back.
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }
}
