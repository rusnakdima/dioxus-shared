//! Layout schema types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Layout {
    pub id: String,
    pub name: String,
    pub layout_type: LayoutType,
    #[serde(default)]
    pub regions: HashMap<String, LayoutRegion>,
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LayoutType {
    Single,
    TwoColumn,
    ThreeColumn,
    Sidebar,
    Dashboard,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LayoutRegion {
    pub id: String,
    pub position: RegionPosition,
    #[serde(default)]
    pub components: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RegionPosition {
    pub column: u32,
    pub row: u32,
    pub col_span: u32,
    pub row_span: u32,
}
