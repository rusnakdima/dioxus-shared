//! Dynamic entity registry
//!
//! Runtime registry of entity schemas. Entity documents are validated
//! against the JSON Schema stored in each `EntitySchema`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Describes a dynamic entity: its name, backing collection, and the JSON
/// Schema used for runtime validation of entity documents.
///
/// Construct with [`EntitySchema::new`]. Validation rules live inside the
/// JSON Schema stored in the `schema` field, so no extra validator hook is
/// needed on the struct itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySchema {
    pub name: String,
    pub collection: String,
    pub schema: serde_json::Value,
}

impl EntitySchema {
    /// Creates a new entity schema definition.
    pub fn new(
        name: impl Into<String>,
        collection: impl Into<String>,
        schema: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            collection: collection.into(),
            schema,
        }
    }
}

/// Thread-safe registry of entity schemas keyed by entity name.
///
/// Thread-safe: backed by `Arc<RwLock<HashMap<...>>>` for interior mutability.
/// Callers may `clone()` freely and register new schemas through `&self`.
#[derive(Clone, Default)]
pub struct EntityRegistry {
    schemas: Arc<RwLock<HashMap<String, EntitySchema>>>,
}

/// Global singleton instance of [`EntityRegistry`].
static ENTITY_REGISTRY: OnceLock<EntityRegistry> = OnceLock::new();

impl EntityRegistry {
    /// Get the global [`EntityRegistry`] singleton instance.
    ///
    /// The registry is created on first call and reused for all subsequent calls.
    /// If you need to pre-populate the registry before any consumer calls `global()`,
    /// call [`EntityRegistry::new()`] and store it, then use this accessor for reads.
    pub fn global() -> &'static EntityRegistry {
        ENTITY_REGISTRY.get_or_init(EntityRegistry::default)
    }
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an entity schema. Replaces any existing schema with the same name.
    pub fn register(&self, name: &str, schema: EntitySchema) {
        let mut map = match self.schemas.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        map.insert(name.to_string(), schema);
    }

    /// Get the schema registered under the given name, if any.
    pub fn get(&self, name: &str) -> Option<EntitySchema> {
        let map = match self.schemas.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        map.get(name).cloned()
    }

    /// Check if a schema is registered under the given name.
    pub fn contains(&self, name: &str) -> bool {
        let map = match self.schemas.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        map.contains_key(name)
    }

    /// Get a list of all registered entity names (sorted).
    pub fn names(&self) -> Vec<String> {
        let map = match self.schemas.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut names: Vec<String> = map.keys().cloned().collect();
        names.sort();
        names
    }

    /// Number of registered entity schemas.
    pub fn len(&self) -> usize {
        let map = match self.schemas.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        map.len()
    }

    /// Returns true if the registry has no registered entity schemas.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema(name: &str, collection: &str) -> EntitySchema {
        EntitySchema {
            name: name.to_string(),
            collection: collection.to_string(),
            schema: serde_json::json!({ "type": "object" }),
        }
    }

    #[test]
    fn test_new_registry_is_empty() {
        let reg = EntityRegistry::new();
        assert_eq!(reg.len(), 0);
        assert!(reg.names().is_empty());
    }

    #[test]
    fn test_register_and_get_roundtrip() {
        let reg = EntityRegistry::new();
        let user_schema = schema("user", "users");
        reg.register("user", user_schema.clone());
        let fetched = reg.get("user").expect("schema should be registered");
        assert_eq!(fetched.name, "user");
        assert_eq!(fetched.collection, "users");
        assert_eq!(fetched.schema, serde_json::json!({ "type": "object" }));
    }

    #[test]
    fn test_contains_after_register() {
        let reg = EntityRegistry::new();
        assert!(!reg.contains("user"));
        reg.register("user", schema("user", "users"));
        assert!(reg.contains("user"));
    }

    #[test]
    fn test_duplicate_register_replaces_previous() {
        let reg = EntityRegistry::new();
        reg.register("user", schema("user", "users_v1"));
        reg.register("user", schema("user", "users_v2"));
        assert_eq!(reg.len(), 1);
        let fetched = reg.get("user").expect("schema should be registered");
        assert_eq!(fetched.collection, "users_v2");
    }

    #[test]
    fn test_names_sorted() {
        let reg = EntityRegistry::new();
        reg.register("zeta", schema("zeta", "zeta_collection"));
        reg.register("alpha", schema("alpha", "alpha_collection"));
        reg.register("mid", schema("mid", "mid_collection"));
        let names = reg.names();
        assert_eq!(names, vec!["alpha", "mid", "zeta"]);
    }

    #[test]
    fn test_register_complex_nested_schema() {
        let reg = EntityRegistry::new();
        let nested_schema = serde_json::json!({
          "type": "object",
          "properties": {
            "id": { "type": "string", "format": "uuid" },
            "profile": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "avatar": { "type": "string", "format": "uri" },
                "metadata": {
                  "type": "object",
                  "additionalProperties": { "type": "string" }
                }
              },
              "required": ["name"]
            },
            "tags": {
              "type": "array",
              "items": { "type": "string" }
            }
          },
          "required": ["id", "profile"]
        });
        let entity_schema = EntitySchema::new("product", "products", nested_schema);
        reg.register("product", entity_schema.clone());
        let fetched = reg.get("product").expect("schema should be registered");
        assert_eq!(fetched.name, "product");
        assert_eq!(fetched.collection, "products");
        assert_eq!(fetched.schema, entity_schema.schema);
    }

    #[test]
    fn test_get_returns_exact_registered_schema() {
        let reg = EntityRegistry::new();
        let schema_obj = serde_json::json!({
          "type": "object",
          "properties": {
            "name": { "type": "string" },
            "count": { "type": "integer", "minimum": 0 }
          }
        });
        let original = EntitySchema::new("item", "items", schema_obj);
        reg.register("item", original.clone());
        let fetched = reg.get("item").expect("schema should exist");
        assert_eq!(fetched.name, original.name);
        assert_eq!(fetched.collection, original.collection);
        assert_eq!(fetched.schema, original.schema);
        assert!(!std::ptr::eq(&fetched, &original));
    }

    #[test]
    fn test_names_returns_all_registered() {
        let reg = EntityRegistry::new();
        assert!(reg.names().is_empty());
        reg.register("one", schema("one", "one_coll"));
        assert_eq!(reg.names(), vec!["one"]);
        reg.register("two", schema("two", "two_coll"));
        reg.register("three", schema("three", "three_coll"));
        let names = reg.names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"one".to_string()));
        assert!(names.contains(&"two".to_string()));
        assert!(names.contains(&"three".to_string()));
    }

    #[test]
    fn test_registry_stores_schema_without_validation() {
        let reg = EntityRegistry::new();
        let invalid_schema = serde_json::json!({
          "type": "invalid_type",
          "properties": {
            "field": { "type": "unknown_type" }
          }
        });
        let entity_schema = EntitySchema::new("broken", "broken_coll", invalid_schema);
        reg.register("broken", entity_schema);
        let fetched = reg
            .get("broken")
            .expect("schema should be stored despite being invalid");
        assert_eq!(fetched.name, "broken");
        assert_eq!(
            fetched.schema,
            serde_json::json!({
              "type": "invalid_type",
              "properties": {
                "field": { "type": "unknown_type" }
              }
            })
        );
    }

    #[test]
    fn test_duplicate_registration_replaces_and_returns_new() {
        let reg = EntityRegistry::new();
        let original = EntitySchema::new(
            "widget",
            "widgets_v1",
            serde_json::json!({"type": "object"}),
        );
        reg.register("widget", original);
        let first = reg.get("widget").expect("first schema should exist");
        assert_eq!(first.collection, "widgets_v1");
        let replacement = EntitySchema::new(
            "widget",
            "widgets_v2",
            serde_json::json!({"type": "object", "properties": {}}),
        );
        reg.register("widget", replacement);
        let second = reg.get("widget").expect("replacement schema should exist");
        assert_eq!(second.collection, "widgets_v2");
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_multiple_entities_independent() {
        let reg = EntityRegistry::new();
        let user = EntitySchema::new("user", "users", serde_json::json!({"type": "object"}));
        let order = EntitySchema::new("order", "orders", serde_json::json!({"type": "object"}));
        let product =
            EntitySchema::new("product", "products", serde_json::json!({"type": "object"}));
        reg.register("user", user);
        reg.register("order", order);
        reg.register("product", product);
        assert_eq!(reg.len(), 3);
        assert!(reg.contains("user"));
        assert!(reg.contains("order"));
        assert!(reg.contains("product"));
        assert!(reg.get("user").is_some());
        assert!(reg.get("order").is_some());
        assert!(reg.get("product").is_some());
        assert_eq!(reg.get("user").unwrap().collection, "users");
        assert_eq!(reg.get("order").unwrap().collection, "orders");
        assert_eq!(reg.get("product").unwrap().collection, "products");
    }
}
