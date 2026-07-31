//! Base CRUD service implementation.

use crate::crud::PaginatedResult;
use crate::response::Response;
use nosql_orm::prelude::*;
use serde_json::Value;
use std::sync::Arc;

/// Base CRUD service that provides standard database operations.
pub struct BaseCrudService {
  provider: Arc<JsonProvider>,
}

impl BaseCrudService {
  pub fn new(provider: Arc<JsonProvider>) -> Self {
    Self { provider }
  }

  pub async fn get(&self, collection: &str, id: &str) -> Result<Response<Value>, String> {
    let result = self
      .provider
      .find_by_id(collection, id)
      .await
      .map_err(|e| e.to_string())?;
    match result {
      Some(data) => Ok(Response::success(data, Some("Found"))),
      None => Ok(Response::not_found(collection)),
    }
  }

  pub async fn get_all(&self, collection: &str) -> Result<Response<Value>, String> {
    let results = self
      .provider
      .find_all(collection)
      .await
      .map_err(|e| e.to_string())?;
    Ok(Response::success(Value::Array(results), Some("Found")))
  }

  pub async fn create(&self, collection: &str, data: Value) -> Result<Response<Value>, String> {
    let result = self
      .provider
      .insert(collection, data)
      .await
      .map_err(|e| e.to_string())?;
    Ok(Response::created(result))
  }

  pub async fn update(&self, collection: &str, id: &str, data: Value) -> Result<Response<Value>, String> {
    let result = self
      .provider
      .update(collection, id, data)
      .await
      .map_err(|e| e.to_string())?;
    Ok(Response::updated(result))
  }

  pub async fn delete(&self, collection: &str, id: &str) -> Result<Response<Value>, String> {
    self
      .provider
      .delete(collection, id)
      .await
      .map_err(|e| e.to_string())?;
    Ok(Response::deleted(Value::Null))
  }

  pub async fn count(&self, collection: &str) -> Result<Response<Value>, String> {
    let count = self
      .provider
      .count(collection, None)
      .await
      .map_err(|e| e.to_string())?;
    Ok(Response::success(
      Value::Number(count.into()),
      Some("Count"),
    ))
  }

  pub async fn paginate(
    &self,
    collection: &str,
    page: u64,
    page_size: u64,
  ) -> Result<Response<PaginatedResult<Value>>, String> {
    let results = self
      .provider
      .find_all(collection)
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

  async fn make_test_provider() -> (TempDir, Arc<JsonProvider>) {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();
    let provider = Arc::new(JsonProvider::new(path).await.unwrap());
    (temp_dir, provider)
  }

  #[tokio::test]
  async fn test_crud_create_and_read() {
    let (_dir, provider) = make_test_provider().await;
    let service = BaseCrudService::new(provider.clone());

    let data = serde_json::json!({"name": "test_item", "value": 42});
    let created = service.create("items", data.clone()).await;
    assert!(created.is_ok());
    let resp = created.unwrap();
    assert_eq!(resp.status, crate::response::Status::Created);
    assert!(resp.data.is_some());

    // The created item has an id
    let item = resp.data.unwrap();
    let id = item.get("id").and_then(|v| v.as_str()).unwrap();

    // read it back
    let read = service.get("items", id).await;
    assert!(read.is_ok());
    let resp = read.unwrap();
    assert_eq!(resp.status, crate::response::Status::Success);
    let retrieved = resp.data.unwrap();
    assert_eq!(retrieved.get("name").and_then(|v| v.as_str()), Some("test_item"));
  }

  #[tokio::test]
  async fn test_crud_read_all() {
    let (_dir, provider) = make_test_provider().await;
    let service = BaseCrudService::new(provider.clone());

    // Initially empty
    let all = service.get_all("items").await;
    assert!(all.is_ok());
    let resp = all.unwrap();
    assert_eq!(resp.status, crate::response::Status::Success);
    let items = resp.data.unwrap();
    let arr = items.as_array().unwrap();
    assert!(arr.is_empty());

    // Create two items
    service.create("items", serde_json::json!({"name": "a"})).await.unwrap();
    service.create("items", serde_json::json!({"name": "b"})).await.unwrap();

    let all = service.get_all("items").await.unwrap();
    let items = all.data.unwrap();
    let arr = items.as_array().unwrap();
    assert_eq!(arr.len(), 2);
  }

  #[tokio::test]
  async fn test_crud_update() {
    let (_dir, provider) = make_test_provider().await;
    let service = BaseCrudService::new(provider.clone());

    let created = service.create("items", serde_json::json!({"name": "original"})).await.unwrap();
    let created_item = created.data.unwrap();
    let id = created_item.get("id").and_then(|v| v.as_str()).unwrap();

    let updated = service.update("items", id, serde_json::json!({"name": "modified"})).await;
    assert!(updated.is_ok());
    assert_eq!(updated.unwrap().status, crate::response::Status::Updated);

    let read = service.get("items", id).await.unwrap();
    let item = read.data.unwrap();
    assert_eq!(item.get("name").and_then(|v| v.as_str()), Some("modified"));
  }

  #[tokio::test]
  async fn test_crud_delete() {
    let (_dir, provider) = make_test_provider().await;
    let service = BaseCrudService::new(provider.clone());

    let created = service.create("items", serde_json::json!({"name": "todelete"})).await.unwrap();
    let created_item = created.data.unwrap();
    let id = created_item.get("id").and_then(|v| v.as_str()).unwrap();

    let deleted = service.delete("items", id).await;
    assert!(deleted.is_ok());
    assert_eq!(deleted.unwrap().status, crate::response::Status::Deleted);

    let read = service.get("items", id).await.unwrap();
    assert_eq!(read.status, crate::response::Status::NotFound);
  }

  #[tokio::test]
  async fn test_crud_count() {
    let (_dir, provider) = make_test_provider().await;
    let service = BaseCrudService::new(provider.clone());

    let count0 = service.count("items").await.unwrap();
    let n0 = count0.data.unwrap().as_i64().unwrap();
    assert_eq!(n0, 0);

    service.create("items", serde_json::json!({"x": 1})).await.unwrap();
    let count1 = service.count("items").await.unwrap();
    let n1 = count1.data.unwrap().as_i64().unwrap();
    assert_eq!(n1, 1);

    service.create("items", serde_json::json!({"x": 2})).await.unwrap();
    let count2 = service.count("items").await.unwrap();
    let n2 = count2.data.unwrap().as_i64().unwrap();
    assert_eq!(n2, 2);
  }
}
