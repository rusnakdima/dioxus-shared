//! Theme schema types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Theme {
  pub id: String,
  pub name: String,
  #[serde(default)]
  pub extends: Option<String>,
  #[serde(default)]
  pub colors: HashMap<String, String>,
}

pub fn get_light_theme() -> Theme {
  Theme {
    id: "light".to_string(),
    name: "Light".to_string(),
    extends: None,
    colors: HashMap::new(),
  }
}

pub fn get_dark_theme() -> Theme {
  Theme {
    id: "dark".to_string(),
    name: "Dark".to_string(),
    extends: None,
    colors: HashMap::new(),
  }
}
