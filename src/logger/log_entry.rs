//! Log entry structure.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub level: super::LogLevel,
    pub message: String,
    pub timestamp: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}
