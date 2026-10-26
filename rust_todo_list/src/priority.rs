use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
