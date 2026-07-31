//! Schema synchronization service.

use std::sync::Arc;
use nosql_orm::prelude::DatabaseProvider;

pub struct SchemaSyncService {
  provider: Arc<nosql_orm::prelude::JsonProvider>,
}

impl SchemaSyncService {
  pub fn new(provider: Arc<nosql_orm::prelude::JsonProvider>) -> Self {
    Self { provider }
  }

  pub async fn sync_schema(&self, _schema_name: &str, schema_data: serde_json::Value) -> anyhow::Result<()> {
    DatabaseProvider::insert(&*self.provider, "schemas", schema_data).await?;
    Ok(())
  }
}
