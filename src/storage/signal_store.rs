use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

type Subscriber = Box<dyn Fn(&str, &Value) + Send + Sync>;
type SubscriberList = RwLock<Vec<Subscriber>>;

/// Thread-safe reactive store for UI state.
pub struct SignalStore {
  data: RwLock<HashMap<String, Value>>,
  subscribers: SubscriberList,
}

impl SignalStore {
  pub fn new() -> Self {
    Self {
      data: RwLock::new(HashMap::new()),
      subscribers: RwLock::new(Vec::new()),
    }
  }

  pub fn set(&self, key: &str, value: Value) {
    {
      let mut data = match self.data.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
      };
      data.insert(key.to_string(), value.clone());
    }
    self.notify(key, &value);
  }

  pub fn get(&self, key: &str) -> Option<Value> {
    let data = match self.data.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    data.get(key).cloned()
  }

  pub fn update<F>(&self, key: &str, f: F)
  where
    F: FnOnce(&Value) -> Value,
  {
    let value = {
      let data = match self.data.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
      };
      data.get(key).cloned()
    };
    if let Some(v) = value {
      let new_value = f(&v);
      self.set(key, new_value);
    }
  }

  pub fn delete(&self, key: &str) {
    let mut data = match self.data.write() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    data.remove(key);
  }

  pub fn keys(&self) -> Vec<String> {
    let data = match self.data.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    data.keys().cloned().collect()
  }

  pub fn subscribe<F>(&self, callback: F)
  where
    F: Fn(&str, &Value) + Send + Sync + 'static,
  {
    let mut subscribers = match self.subscribers.write() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    subscribers.push(Box::new(callback));
  }

  fn notify(&self, key: &str, value: &Value) {
    let subscribers = match self.subscribers.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    for subscriber in subscribers.iter() {
      subscriber(key, value);
    }
  }

  pub fn to_json(&self) -> Value {
    let data = match self.data.read() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };
    Value::Object(data.clone().into_iter().collect())
  }

  pub fn from_json(&self, json: Value) {
    if let Value::Object(map) = json {
      let mut data = match self.data.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
      };
      *data = map.into_iter().collect();
    }
  }
}

impl Default for SignalStore {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_signal_store_set_get() {
    let store = SignalStore::new();
    store.set("key", serde_json::json!("value"));
    assert_eq!(store.get("key"), Some(serde_json::json!("value")));
  }

  #[test]
  fn test_signal_store_update() {
    let store = SignalStore::new();
    store.set("counter", serde_json::json!(0));
    store.update("counter", |v| serde_json::json!(v.as_i64().unwrap() + 1));
    assert_eq!(store.get("counter"), Some(serde_json::json!(1)));
  }

  #[test]
  fn test_signal_store_delete() {
    let store = SignalStore::new();
    store.set("key", serde_json::json!("value"));
    store.delete("key");
    assert_eq!(store.get("key"), None);
  }

  #[test]
  fn test_signal_store_subscription_notifications() {
    use std::sync::{Arc, Mutex};

    let store = SignalStore::new();
    let notifications: Arc<Mutex<Vec<(String, serde_json::Value)>>> =
      Arc::new(Mutex::new(Vec::new()));
    let notifications_clone = notifications.clone();

    store.subscribe(move |key, value| {
      if let Ok(mut notifications) = notifications_clone.lock() {
        notifications.push((key.to_string(), value.clone()));
      }
    });
    store.set("key1", serde_json::json!("value1"));
    store.set("key2", serde_json::json!("value2"));

    let notifications = notifications.lock().unwrap();
    assert_eq!(notifications.len(), 2);
  }

  #[test]
  fn test_signal_store_keys() {
    let store = SignalStore::new();
    store.set("key1", serde_json::json!("value1"));
    store.set("key2", serde_json::json!("value2"));
    let keys = store.keys();
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
  }
}
