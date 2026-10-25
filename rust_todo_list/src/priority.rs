use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}
