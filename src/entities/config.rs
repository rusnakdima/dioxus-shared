//! AppConfig entity

use crate::themes::ThemeVariant;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppConfig {
    pub app_name: String,
    pub api_base_url: String,
    pub theme_default: ThemeVariant,
    pub log_level: String,
}
