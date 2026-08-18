//! CRUD service for database operations.

use crate::crud::types::PaginatedResult;
use crate::response::Response;
use anyhow::Result;
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
                self.provider
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
    use anyhow::Context;
    use tempfile::TempDir;

    async fn make_test_provider(path: &std::path::Path) -> Result<Arc<JsonProvider>> {
        let provider = JsonProvider::new(path)
            .await
            .context("failed to create JsonProvider")?;
        Ok(Arc::new(provider))
    }

    #[tokio::test]
    async fn test_crud_get_all() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(
            &items_path,
            r#"[{"id": "1", "name": "Test Item", "value": 42}]"#,
        )
        .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud.execute("get_all", "items", None, None, None).await;
        assert!(result.is_ok());
        let resp = result.map_err(|e| anyhow::anyhow!(e))?;
        assert!(resp.data.is_some(), "data must exist for get_all result");
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_count() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(&items_path, r#"[{"id": "1", "name": "Test Item"}]"#)
            .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud.execute("count", "items", None, None, None).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_get() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(
            &items_path,
            r#"[{"id": "1", "name": "Test Item", "value": 42}]"#,
        )
        .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud
            .execute("get", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Success);
        let data = result.data.context("data must exist")?;
        assert_eq!(data.get("name").and_then(|v| v.as_str()), Some("Test Item"));
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_get_nonexistent() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(&items_path, r#"[{"id": "1", "name": "Test Item"}]"#)
            .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud
            .execute("get", "items", Some("nonexistent"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::NotFound);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_create() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let data = serde_json::json!({"name": "New Item", "value": 100});
        let result = crud
            .execute("create", "items", None, Some(data), None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Created);
        let created = result.data.context("data must exist")?;
        let id = created
            .get("id")
            .and_then(|v| v.as_str())
            .context("created item missing id")?;
        assert_eq!(
            created.get("name").and_then(|v| v.as_str()),
            Some("New Item")
        );

        let read = crud
            .execute("get", "items", Some(id), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(read.status, crate::response::Status::Success);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_update() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(&items_path, r#"[{"id": "1", "name": "Original"}]"#)
            .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let data = serde_json::json!({"name": "Updated"});
        let result = crud
            .execute("update", "items", Some("1"), Some(data), None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Updated);

        let read = crud
            .execute("get", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(read.status, crate::response::Status::Success);
        let data = read.data.context("data must exist")?;
        assert_eq!(data.get("name").and_then(|v| v.as_str()), Some("Updated"));
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_patch() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(
            &items_path,
            r#"[{"id": "1", "name": "Original", "value": 10}]"#,
        )
        .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let patch_data = serde_json::json!({"value": 20});
        let result = crud
            .execute("patch", "items", Some("1"), Some(patch_data), None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Updated);

        let read = crud
            .execute("get", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(read.status, crate::response::Status::Success);
        let data = read.data.context("data must exist")?;
        assert_eq!(data.get("name").and_then(|v| v.as_str()), Some("Original"));
        assert_eq!(data.get("value").and_then(|v| v.as_i64()), Some(20));
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_delete() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(&items_path, r#"[{"id": "1", "name": "To Delete"}]"#)
            .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud
            .execute("delete", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Deleted);

        let read = crud
            .execute("get", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(read.status, crate::response::Status::NotFound);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_exists() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        std::fs::write(&items_path, r#"[{"id": "1", "name": "Test Item"}]"#)
            .context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));
        let result = crud
            .execute("exists", "items", Some("1"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result.status, crate::response::Status::Success);
        assert_eq!(
            result.data.context("data must exist")?.as_bool(),
            Some(true)
        );

        let result_nonexistent = crud
            .execute("exists", "items", Some("nonexistent"), None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(result_nonexistent.status, crate::response::Status::Success);
        assert_eq!(
            result_nonexistent
                .data
                .context("data must exist")?
                .as_bool(),
            Some(false)
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_paginate() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let items_path = temp_dir.path().join("items.json");
        let items: Vec<serde_json::Value> = (0..5)
            .map(|i| serde_json::json!({"id": format!("{}", i + 1), "name": format!("Item {}", i)}))
            .collect();
        let items_json = serde_json::to_string(&items).context("failed to serialize items")?;
        std::fs::write(&items_path, items_json).context("failed to write items.json")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));

        let page1 = crud
            .paginate("items", 1, 2)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(page1.data.as_ref().unwrap().items.len(), 2);
        assert!(page1.data.as_ref().unwrap().has_more);
        assert_eq!(page1.data.as_ref().unwrap().total_count, 5);

        let page2 = crud
            .paginate("items", 2, 2)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(page2.data.as_ref().unwrap().items.len(), 2);
        assert!(page2.data.as_ref().unwrap().has_more);

        let page3 = crud
            .paginate("items", 3, 2)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(page3.data.as_ref().unwrap().items.len(), 1);
        assert!(!page3.data.as_ref().unwrap().has_more);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_create_persists_multiple_items() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let crud = CrudService::new(Arc::clone(&provider));

        crud.execute(
            "create",
            "items",
            None,
            Some(serde_json::json!({"name": "first", "value": 100})),
            None,
        )
        .await
        .map_err(anyhow::Error::msg)?;
        crud.execute(
            "create",
            "items",
            None,
            Some(serde_json::json!({"name": "second", "value": 200})),
            None,
        )
        .await
        .map_err(anyhow::Error::msg)?;
        crud.execute(
            "create",
            "items",
            None,
            Some(serde_json::json!({"name": "third", "value": 300})),
            None,
        )
        .await
        .map_err(anyhow::Error::msg)?;

        let count = crud
            .execute("count", "items", None, None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(
            count
                .data
                .context("data must exist")?
                .as_i64()
                .context("count data is not a valid i64")?,
            3
        );

        let all = crud
            .execute("get_all", "items", None, None, None)
            .await
            .map_err(anyhow::Error::msg)?;
        let out = all;
        let data = out.data.context("data must exist")?;
        let items = data.as_array().context("data is not an array")?;
        assert_eq!(items.len(), 3);
        Ok(())
    }
}
