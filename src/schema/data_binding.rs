//! Data binding schema types.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DataBinding {
  pub store: String,
  pub field: String,
  #[serde(default)]
  pub transform: Option<String>,
  #[serde(default)]
  pub validator: Option<String>,
}

impl DataBinding {
  pub fn new(store: &str, field: &str) -> Self {
    Self {
      store: store.to_string(),
      field: field.to_string(),
      transform: None,
      validator: None,
    }
  }
}
