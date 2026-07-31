//! CRUD service for database operations.

use crate::response::Response;
use crate::crud::types::PaginatedResult;
use nosql_orm::prelude::*;
use serde_json::Value;
use std::sync::Arc;

pub struct CrudService {
  provider: Arc<JsonProvider>,
}

impl CrudService {
  pub fn new(provider: Arc<JsonProvider>) -> Self {
    Self { provider }
  }

  pub async fn execute(
    &self,
    operation: &str,
    entity: &str,
    id: Option<&str>,
    data: Option<Value>,
    _filter: Option<Value>,
  ) -> Result<Response<Value>, String> {
    match operation {
      "get" => {
        let id = id.ok_or("ID required for get")?;
        let result = self
          .provider
          .find_by_id(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        match result {
          Some(data) => Ok(Response::success(data, Some("Found"))),
          None => Ok(Response::not_found(entity)),
        }
      }
      "get_all" => {
        let results = self
          .provider
          .find_all(entity)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(Value::Array(results), Some("Found")))
      }
      "create" | "save" => {
        let data = data.ok_or("Data required for create")?;
        let result = self
          .provider
          .insert(entity, data)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::created(result))
      }
      "update" => {
        let id = id.ok_or("ID required for update")?;
        let mut data = data.ok_or("Data required for update")?;
        if let Some(obj) = data.as_object_mut() {
          obj.insert("id".to_string(), Value::String(id.to_string()));
        }
        let result = self
          .provider
          .update(entity, id, data)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::updated(result))
      }
      "patch" => {
        let id = id.ok_or("ID required for patch")?;
        let patch = data.ok_or("Patch data required")?;
        let result = self
          .provider
          .patch(entity, id, patch)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::updated(result))
      }
      "delete" => {
        let id = id.ok_or("ID required for delete")?;
        self
          .provider
          .delete(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::deleted(Value::Null))
      }
      "count" => {
        let count = self
          .provider
          .count(entity, None)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(
          Value::Number(count.into()),
          Some("Count"),
        ))
      }
      "exists" => {
        let id = id.ok_or("ID required for exists")?;
        let exists = self
          .provider
          .exists(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(
          Value::Bool(exists),
          Some(if exists { "Exists" } else { "Not found" }),
        ))
      }
      _ => Err(format!("Unknown operation: {}", operation)),
    }
  }

  pub async fn paginate(
    &self,
    entity: &str,
    page: u64,
    page_size: u64,
  ) -> Result<Response<PaginatedResult<Value>>, String> {
    let results = self
      .provider
      .find_all(entity)
      .await
      .map_err(|e| e.to_string())?;

    let total_count = results.len();
    let start = ((page.saturating_sub(1)) * page_size) as usize;
    let end = start + page_size as usize;

    let items: Vec<Value> = if start < results.len() {
      results[start..results.len().min(end)].to_vec()
    } else {
      vec![]
    };

    Ok(Response::success(
      PaginatedResult {
        items,
        has_more: end < total_count,
        total_count,
      },
      Some("Paginated"),
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;

  #[tokio::test]
  async fn test_crud_get_all() {
    let temp_dir = TempDir::new().unwrap();
    let items_path = temp_dir.path().join("items.json");
    std::fs::write(
      &items_path,
      r#"[{"id": "1", "name": "Test Item", "value": 42}]"#,
    )
    .unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let crud = CrudService::new(Arc::new(provider));
    let result = crud.execute("get_all", "items", None, None, None).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.data.expect("data must exist").is_array());
  }

  #[tokio::test]
  async fn test_crud_count() {
    let temp_dir = TempDir::new().unwrap();
    let items_path = temp_dir.path().join("items.json");
    std::fs::write(&items_path, r#"[{"id": "1", "name": "Test Item"}]"#).unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let crud = CrudService::new(Arc::new(provider));
    let result = crud.execute("count", "items", None, None, None).await;
    assert!(result.is_ok());
  }
}
