//! Dynamic entity registry
//!
//! Runtime registry of entity schemas. Entity documents are validated
//! against the JSON Schema stored in each `EntitySchema`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Describes a dynamic entity: its name, backing collection, and the JSON
/// Schema used for runtime validation of entity documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySchema {
  pub name: String,
  pub collection: String,
  pub schema: serde_json::Value,
}

/// Thread-safe registry of entity schemas keyed by entity name.
///
/// Thread-safe: backed by `Arc<RwLock<HashMap<...>>>` for interior mutability.
/// Callers may `clone()` freely and register new schemas through `&self`.
#[derive(Clone, Default)]
pub struct EntityRegistry {
  schemas: Arc<RwLock<HashMap<String, EntitySchema>>>,
}

impl EntityRegistry {
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
}
