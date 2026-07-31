//! Page schema types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::data_binding::DataBinding;
use super::element_layout::ElementLayout;
use super::handlers::{ActionDef, DataSourceDef};

/// Runtime schema container
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Schema {
    pub app_id: String,
    pub version: String,
    #[serde(default)]
    pub pages: Vec<Page>,
    #[serde(default)]
    pub shortcuts: Vec<Shortcut>,
    #[serde(default)]
    pub modals: Vec<Modal>,
}

/// Keyboard shortcut definition
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Shortcut {
    pub id: String,
    pub keys: String,
    pub action: String,
}

/// Modal dialog definition
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Modal {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub elements: Vec<CanvasElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Page {
  pub id: String,
  pub title: String,
  pub description: Option<String>,
  pub route: String,
  #[serde(default)]
  pub layout: String,
  #[serde(default)]
  pub elements: Vec<CanvasElement>,
  #[serde(default)]
  pub meta: PageMeta,
  #[serde(default)]
  pub sections: HashMap<String, PageSection>,
  #[serde(default)]
  pub layout_mode: Option<String>,
  #[serde(default)]
  pub data_sources: Option<Vec<DataSourceDef>>,
  #[serde(default)]
  pub actions: Option<Vec<ActionDef>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct PageMeta {
  #[serde(default)]
  pub title: Option<String>,
  #[serde(default)]
  pub icon: Option<String>,
  #[serde(default)]
  pub breadcrumb: Vec<String>,
  #[serde(default)]
  pub description: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PageSection {
  #[serde(default)]
  pub component_id: Option<String>,
  #[serde(default = "default_visible")]
  pub visible: bool,
  #[serde(default)]
  pub dynamic: bool,
}

fn default_visible() -> bool {
  true
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub struct CanvasElement {
  pub id: String,
  #[serde(alias = "componentId")]
  pub component: String,
  #[serde(default)]
  pub grid_position: GridPosition,
  #[serde(default)]
  pub props: HashMap<String, serde_json::Value>,
  #[serde(default)]
  pub classes: String,
  #[serde(default)]
  pub children: Vec<CanvasElement>,
  #[serde(default)]
  pub data_binding: Option<DataBinding>,
  #[serde(default)]
  pub type_field: Option<String>,
  #[serde(default = "default_visible_true")]
  pub visible: bool,
  #[serde(default)]
  pub layout: Option<ElementLayout>,
}

fn default_visible_true() -> bool {
  true
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GridPosition {
  #[serde(default = "default_column")]
  pub column: i32,
  #[serde(default = "default_row")]
  pub row: i32,
  #[serde(default = "default_col_span")]
  pub col_span: i32,
  #[serde(default = "default_row_span")]
  pub row_span: i32,
  #[serde(default)]
  pub col_start: Option<i32>,
  #[serde(default)]
  pub row_start: Option<i32>,
}

impl Default for GridPosition {
  fn default() -> Self {
    Self {
      column: 1,
      row: 1,
      col_span: 1,
      row_span: 1,
      col_start: None,
      row_start: None,
    }
  }
}

fn default_column() -> i32 { 1 }
fn default_row() -> i32 { 1 }
fn default_col_span() -> i32 { 1 }
fn default_row_span() -> i32 { 1 }
