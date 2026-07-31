//! Element layout schema types.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ElementLayout {
  #[serde(default)]
  pub display: DisplayType,
  #[serde(default)]
  pub flex_direction: Option<FlexDirection>,
  #[serde(default)]
  pub gap: Option<f32>,
  #[serde(default)]
  pub align_items: Option<AlignItems>,
  #[serde(default)]
  pub justify_content: Option<JustifyContent>,
  #[serde(default)]
  pub padding: Option<Spacing>,
  #[serde(default)]
  pub margin: Option<Spacing>,
  #[serde(default)]
  pub width: Option<String>,
  #[serde(default)]
  pub height: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum DisplayType {
  #[default]
  Flex,
  Grid,
  Block,
  Inline,
  InlineBlock,
  None,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FlexDirection {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  Baseline,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Spacing {
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
  pub left: f32,
}

impl Spacing {
  pub fn all(val: f32) -> Self {
    Self {
      top: val,
      right: val,
      bottom: val,
      left: val,
    }
  }
}
