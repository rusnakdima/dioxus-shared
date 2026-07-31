//! Schema system setup and configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaConfig {
  pub schema_dir: PathBuf,
  pub cache_enabled: bool,
  pub hot_reload: bool,
}

impl Default for SchemaConfig {
  fn default() -> Self {
    Self {
      schema_dir: PathBuf::from("./schemas"),
      cache_enabled: true,
      hot_reload: false,
    }
  }
}

pub struct SchemaSystem {
  config: SchemaConfig,
}

impl SchemaSystem {
  pub fn new(config: SchemaConfig) -> Self {
    Self { config }
  }

  pub fn schema_dir(&self) -> &PathBuf {
    &self.config.schema_dir
  }
}

#[derive(Debug)]
pub struct SchemaSyncState {
  pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
  pub schema_count: usize,
}

pub async fn setup_schema_system(config: SchemaConfig) -> anyhow::Result<SchemaSystem> {
  Ok(SchemaSystem::new(config))
}
