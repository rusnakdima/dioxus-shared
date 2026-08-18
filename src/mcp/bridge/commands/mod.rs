//! Commands available via the MCP Bridge

use serde::{Deserialize, Serialize};

/// Application information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub dioxus_version: String,
}
