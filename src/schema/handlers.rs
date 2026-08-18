//! Handler schema types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ActionDef {
    pub id: String,
    pub name: String,
    pub action_type: ActionType,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Navigate,
    ApiCall,
    StoreUpdate,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DataSourceDef {
    pub id: String,
    pub source_type: DataSourceType,
    pub endpoint: Option<String>,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DataSourceType {
    Api,
    Store,
    Static,
    Computed,
}
