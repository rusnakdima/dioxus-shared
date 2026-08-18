//! Base CRUD service implementation.

use crate::crud::PaginatedResult;
use crate::response::Response;
use anyhow::Result;
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

    pub async fn update(
        &self,
        collection: &str,
        id: &str,
        data: Value,
    ) -> Result<Response<Value>, String> {
        let result = self
            .provider
            .update(collection, id, data)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Response::updated(result))
    }

    pub async fn delete(&self, collection: &str, id: &str) -> Result<Response<Value>, String> {
        self.provider
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
    use anyhow::Context;
    use tempfile::TempDir;

    async fn make_test_provider(path: &std::path::Path) -> Result<Arc<JsonProvider>> {
        let provider = JsonProvider::new(path)
            .await
            .context("failed to create JsonProvider")?;
        Ok(Arc::new(provider))
    }

    #[tokio::test]
    async fn test_crud_create_and_read() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let data = serde_json::json!({"name": "test_item", "value": 42});
        let created = service
            .create("items", data.clone())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let resp = created;
        assert_eq!(resp.status, crate::response::Status::Created);
        assert!(resp.data.is_some());

        // The created item has an id
        let item = resp.data.unwrap();
        let id = item
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("created item missing id field"))?;

        // read it back
        let read = service
            .get("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let resp = read;
        assert_eq!(resp.status, crate::response::Status::Success);
        let retrieved = resp.data.unwrap();
        assert_eq!(
            retrieved.get("name").and_then(|v| v.as_str()),
            Some("test_item")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_read_all() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        // Initially empty
        let all = service
            .get_all("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let resp = all;
        assert_eq!(resp.status, crate::response::Status::Success);
        let items = resp.data.unwrap();
        let arr = items
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("data is not an array"))?;
        assert!(arr.is_empty());

        // Create two items
        service
            .create("items", serde_json::json!({"name": "a"}))
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        service
            .create("items", serde_json::json!({"name": "b"}))
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let all = service
            .get_all("items")
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let items = all.data.unwrap();
        let arr = items
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("data is not an array"))?;
        assert_eq!(arr.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_update() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let created = service
            .create("items", serde_json::json!({"name": "original"}))
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let created_item = created.data.unwrap();
        let id = created_item
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("created item missing id field"))?;

        let updated = service
            .update("items", id, serde_json::json!({"name": "modified"}))
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        assert_eq!(updated.status, crate::response::Status::Updated);

        let read = service
            .get("items", id)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let item = read.data.unwrap();
        assert_eq!(item.get("name").and_then(|v| v.as_str()), Some("modified"));
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_delete() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let created = service
            .create("items", serde_json::json!({"name": "todelete"}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let created_item = created.data.unwrap();
        let id = created_item
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("created item missing id field"))?;

        let deleted = service
            .delete("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(deleted.status, crate::response::Status::Deleted);

        let read = service
            .get("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(read.status, crate::response::Status::NotFound);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_count() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let count0 = service
            .count("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let n0 = count0
            .data
            .unwrap()
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("count0 data is not a valid i64"))?;
        assert_eq!(n0, 0);

        service
            .create("items", serde_json::json!({"x": 1}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let count1 = service
            .count("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let n1 = count1
            .data
            .unwrap()
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("count1 data is not a valid i64"))?;
        assert_eq!(n1, 1);

        service
            .create("items", serde_json::json!({"x": 2}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let count2 = service
            .count("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let n2 = count2
            .data
            .unwrap()
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("count2 data is not a valid i64"))?;
        assert_eq!(n2, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_pagination() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        for i in 0..5 {
            service
                .create(
                    "items",
                    serde_json::json!({"name": format!("item_{}", i), "value": i}),
                )
                .await
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }

        let page1 = service
            .paginate("items", 1, 2)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let data1 = page1.data.unwrap();
        assert_eq!(data1.items.len(), 2);
        assert!(data1.has_more);
        assert_eq!(data1.total_count, 5);

        let page2 = service
            .paginate("items", 2, 2)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let data2 = page2.data.unwrap();
        assert_eq!(data2.items.len(), 2);
        assert!(data2.has_more);
        assert_eq!(data2.total_count, 5);

        let page3 = service
            .paginate("items", 3, 2)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let data3 = page3.data.unwrap();
        assert_eq!(data3.items.len(), 1);
        assert!(!data3.has_more);
        assert_eq!(data3.total_count, 5);

        let page10 = service
            .paginate("items", 10, 2)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let data10 = page10.data.unwrap();
        assert!(data10.items.is_empty());
        assert!(!data10.has_more);
        assert_eq!(data10.total_count, 5);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_read_nonexistent() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let read = service
            .get("items", "nonexistent-id")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(read.status, crate::response::Status::NotFound);
        assert!(read.data.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_delete_verifies_item_gone() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let created = service
            .create("items", serde_json::json!({"name": "to_delete"}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let created_item = created.data.unwrap();
        let id = created_item
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("created item missing id field"))?;

        let read_before = service
            .get("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(read_before.status, crate::response::Status::Success);

        let deleted = service
            .delete("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(deleted.status, crate::response::Status::Deleted);

        let read_after = service
            .get("items", id)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        assert_eq!(read_after.status, crate::response::Status::NotFound);

        let count = service
            .count("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let n = count
            .data
            .unwrap()
            .as_i64()
            .context("count data is not a valid i64")?;
        assert_eq!(n, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_create_persists_multiple_items() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let item1 = service
            .create("items", serde_json::json!({"name": "first", "value": 100}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let item2 = service
            .create("items", serde_json::json!({"name": "second", "value": 200}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let item3 = service
            .create("items", serde_json::json!({"name": "third", "value": 300}))
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let item1_data = item1.data.unwrap();
        let item2_data = item2.data.unwrap();
        let item3_data = item3.data.unwrap();
        let id1 = item1_data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("item1 missing id field"))?;
        let id2 = item2_data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("item2 missing id field"))?;
        let id3 = item3_data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("item3 missing id field"))?;

        let read1 = service
            .get("items", id1)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let read2 = service
            .get("items", id2)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let read3 = service
            .get("items", id3)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let read1_data = read1.data.unwrap();
        let read2_data = read2.data.unwrap();
        let read3_data = read3.data.unwrap();
        assert_eq!(read1_data.get("value").and_then(|v| v.as_i64()), Some(100));
        assert_eq!(read2_data.get("value").and_then(|v| v.as_i64()), Some(200));
        assert_eq!(read3_data.get("value").and_then(|v| v.as_i64()), Some(300));

        let count = service
            .count("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let n = count
            .data
            .unwrap()
            .as_i64()
            .context("count data is not a valid i64")?;
        assert_eq!(n, 3);

        let all = service
            .get_all("items")
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let all_data = all.data.unwrap();
        let items = all_data
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("all data is not an array"))?;
        assert_eq!(items.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_update_partial_data() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        let created = service
            .create(
                "items",
                serde_json::json!({"name": "original", "value": 10, "description": "old desc"}),
            )
            .await
            .map_err(anyhow::Error::msg)?;
        let created_item = created.data.unwrap();
        let id = created_item
            .get("id")
            .and_then(|v| v.as_str())
            .context("created item missing id field")?;

        let updated = service
            .update(
                "items",
                id,
                serde_json::json!({"name": "updated", "value": 20}),
            )
            .await
            .map_err(anyhow::Error::msg)?;
        assert_eq!(updated.status, crate::response::Status::Updated);

        let read = service.get("items", id).await.map_err(anyhow::Error::msg)?;
        let item = read.data.unwrap();
        assert_eq!(item.get("name").and_then(|v| v.as_str()), Some("updated"));
        assert_eq!(item.get("value").and_then(|v| v.as_i64()), Some(20));
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_cross_collection_isolation() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        service
            .create("collection_a", serde_json::json!({"name": "only_in_a"}))
            .await
            .map_err(anyhow::Error::msg)?;
        service
            .create("collection_b", serde_json::json!({"name": "only_in_b"}))
            .await
            .map_err(anyhow::Error::msg)?;

        let all_a = service
            .get_all("collection_a")
            .await
            .map_err(anyhow::Error::msg)?;
        let data_a = all_a.data.unwrap();
        let items_a = data_a
            .as_array()
            .context("collection_a data is not an array")?;
        assert_eq!(items_a.len(), 1);
        assert_eq!(
            items_a[0].get("name").and_then(|v| v.as_str()),
            Some("only_in_a")
        );

        let all_b = service
            .get_all("collection_b")
            .await
            .map_err(anyhow::Error::msg)?;
        let data_b = all_b.data.unwrap();
        let items_b = data_b
            .as_array()
            .context("collection_b data is not an array")?;
        assert_eq!(items_b.len(), 1);
        assert_eq!(
            items_b[0].get("name").and_then(|v| v.as_str()),
            Some("only_in_b")
        );

        let count_a = service
            .count("collection_a")
            .await
            .map_err(anyhow::Error::msg)?;
        let n_a = count_a
            .data
            .unwrap()
            .as_i64()
            .context("count_a data is not a valid i64")?;
        assert_eq!(n_a, 1);

        let count_b = service
            .count("collection_b")
            .await
            .map_err(anyhow::Error::msg)?;
        let n_b = count_b
            .data
            .unwrap()
            .as_i64()
            .context("count_b data is not a valid i64")?;
        assert_eq!(n_b, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_crud_multiple_collections_coexist() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let service = BaseCrudService::new(provider.clone());

        service
            .create("users", serde_json::json!({"email": "a@test.com"}))
            .await
            .map_err(anyhow::Error::msg)?;
        service
            .create("users", serde_json::json!({"email": "b@test.com"}))
            .await
            .map_err(anyhow::Error::msg)?;
        service
            .create("posts", serde_json::json!({"title": "hello"}))
            .await
            .map_err(anyhow::Error::msg)?;

        let user_count = service.count("users").await.map_err(anyhow::Error::msg)?;
        assert_eq!(
            user_count
                .data
                .unwrap()
                .as_i64()
                .context("user_count data is not a valid i64")?,
            2
        );

        let post_count = service.count("posts").await.map_err(anyhow::Error::msg)?;
        assert_eq!(
            post_count
                .data
                .unwrap()
                .as_i64()
                .context("post_count data is not a valid i64")?,
            1
        );
        Ok(())
    }
}
