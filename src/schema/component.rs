//! Component schema types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub handlers: Vec<Handler>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Layout,
    Container,
    Form,
    Display,
    Navigation,
    Feedback,
    Media,
    DataDisplay,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Handler {
    pub event: String,
    pub action: String,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}
