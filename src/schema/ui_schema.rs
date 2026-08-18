//! UI Schema types.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct UiSchema {
    pub version: String,
    pub app: super::AppConfig,
    #[serde(default)]
    pub pages: Vec<super::Page>,
    #[serde(default)]
    pub components: Vec<super::Component>,
    #[serde(default)]
    pub layouts: Vec<super::Layout>,
}

impl UiSchema {
    pub fn new(app: super::AppConfig) -> Self {
        Self {
            version: "1.0.0".to_string(),
            app,
            pages: Vec::new(),
            components: Vec::new(),
            layouts: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: super::Page) {
        self.pages.push(page);
    }

    pub fn add_component(&mut self, component: super::Component) {
        self.components.push(component);
    }

    pub fn add_layout(&mut self, layout: super::Layout) {
        self.layouts.push(layout);
    }
}
