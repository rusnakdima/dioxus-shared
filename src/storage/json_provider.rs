//! JSON Provider wrapper for nosql_orm JsonProvider.

use nosql_orm::prelude::*;
use nosql_orm::providers::json::JsonProviderConfig;
use std::path::Path;
use std::sync::Arc;

/// Type alias for the JSON provider state.
pub type JsonProviderState = Arc<JsonProvider>;

/// Create a new JsonProvider instance with the given data directory.
pub async fn create_json_provider(data_dir: impl AsRef<Path>) -> OrmResult<JsonProvider> {
    JsonProvider::new(data_dir).await
}

/// Create a new JsonProvider instance with custom configuration.
pub async fn create_json_provider_with_config(
    config: JsonProviderConfig,
) -> OrmResult<JsonProvider> {
    JsonProvider::with_config(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_dir() -> std::path::PathBuf {
        let temp = std::env::temp_dir();
        let unique_dir = temp.join(format!("test_provider_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&unique_dir).expect("Failed to create temp directory for test");
        unique_dir
    }

    fn cleanup_temp_dir(path: &std::path::Path) {
        let _ = std::fs::remove_dir_all(path);
    }

    #[tokio::test]
    async fn test_create_provider() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await;
        assert!(provider.is_ok());
        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_find_nonexistent() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();
        let found = provider
            .find_by_id("test_collection", "nonexistent-id")
            .await;
        assert!(found.is_ok());
        assert!(found.unwrap().is_none());
        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_insert_and_find_by_id() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let doc = serde_json::json!({
          "id": "test_doc_id",
          "name": "test_doc",
          "value": 42
        });
        provider.insert("items", doc.clone()).await.unwrap();

        let found = provider.find_by_id("items", "test_doc_id").await.unwrap();
        assert!(found.is_some());

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_update_existing_document() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let original = serde_json::json!({
          "id": "doc1",
          "name": "original"
        });
        provider.insert("docs", original.clone()).await.unwrap();

        let updated = serde_json::json!({
          "id": "doc1",
          "name": "modified"
        });
        provider
            .update("docs", "doc1", updated.clone())
            .await
            .unwrap();

        let found = provider.find_by_id("docs", "doc1").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap()["name"], serde_json::json!("modified"));

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_delete_document() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let doc = serde_json::json!({"id": "todelete", "name": "will_be_deleted"});
        provider.insert("items", doc.clone()).await.unwrap();

        let found_before = provider.find_by_id("items", "todelete").await.unwrap();
        assert!(found_before.is_some());

        provider.delete("items", "todelete").await.unwrap();

        let found_after = provider.find_by_id("items", "todelete").await.unwrap();
        assert!(found_after.is_none());

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_list_collection() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        provider
            .insert("items", serde_json::json!({"id": "1", "name": "first"}))
            .await
            .unwrap();
        provider
            .insert("items", serde_json::json!({"id": "2", "name": "second"}))
            .await
            .unwrap();
        provider
            .insert("items", serde_json::json!({"id": "3", "name": "third"}))
            .await
            .unwrap();

        let all = provider.find_all("items").await.unwrap();
        assert_eq!(all.len(), 3);

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_complex_nested_json() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let nested = serde_json::json!({
          "id": "complex",
          "user": {
            "profile": {
              "name": "Alice",
              "contacts": {
                "email": "alice@example.com",
                "phone": null
              }
            },
            "preferences": {
              "theme": "dark",
              "notifications": true
            }
          },
          "tags": ["admin", "premium", "active"],
          "metadata": {
            "created_at": "2024-01-01",
            "version": 1
          }
        });

        provider.insert("docs", nested.clone()).await.unwrap();

        let found = provider.find_by_id("docs", "complex").await.unwrap();
        assert!(found.is_some());

        let retrieved = found.unwrap();
        assert_eq!(
            retrieved["user"]["profile"]["name"],
            serde_json::json!("Alice")
        );
        assert_eq!(
            retrieved["user"]["profile"]["contacts"]["email"],
            serde_json::json!("alice@example.com")
        );
        assert_eq!(retrieved["tags"][0], serde_json::json!("admin"));
        assert_eq!(retrieved["metadata"]["version"], serde_json::json!(1));

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_empty_collection() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let all = provider.find_all("empty_collection").await.unwrap();
        assert!(all.is_empty());

        cleanup_temp_dir(&dir);
    }

    #[tokio::test]
    async fn test_update_nonexistent_document() {
        let dir = create_temp_dir();
        let provider = create_json_provider(&dir).await.unwrap();

        let result = provider
            .update("docs", "nonexistent", serde_json::json!({"name": "test"}))
            .await;
        assert!(result.is_err());

        cleanup_temp_dir(&dir);
    }
}
