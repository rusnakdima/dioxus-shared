use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

type Subscriber = Box<dyn Fn(&str, &Value) + Send + Sync>;
type SubscriberList = Arc<RwLock<Vec<(usize, Subscriber)>>>;

/// Thread-safe reactive store for UI state.
pub struct SignalStore {
    data: RwLock<HashMap<String, Value>>,
    subscribers: SubscriberList,
    next_id: AtomicUsize,
}

/// Handle returned by [`SignalStore::subscribe`]. Clone to keep subscription alive. Dropping all handles unsubscribes.
#[derive(Clone)]
pub struct SubscriptionHandle {
    id: usize,
    ref_count: Arc<AtomicUsize>,
    subscribers: SubscriberList,
}

impl SubscriptionHandle {
    /// Remove this subscription from the store.
    pub fn unsubscribe(self) {
        if self.ref_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            let mut subs = self.subscribers.write().unwrap();
            subs.retain(|(id, _)| *id != self.id);
        }
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if self.ref_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            let mut subs = self.subscribers.write().unwrap();
            subs.retain(|(id, _)| *id != self.id);
        }
    }
}

impl SignalStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
            subscribers: Arc::new(RwLock::new(Vec::new())),
            next_id: AtomicUsize::new(0),
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

    pub fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(&str, &Value) + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut subscribers = match self.subscribers.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        subscribers.push((id, Box::new(callback)));
        SubscriptionHandle {
            id,
            ref_count: Arc::new(AtomicUsize::new(1)),
            subscribers: Arc::clone(&self.subscribers),
        }
    }

    fn notify(&self, key: &str, value: &Value) {
        let subscribers = match self.subscribers.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        for (_, subscriber) in subscribers.iter() {
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

impl Drop for SignalStore {
    fn drop(&mut self) {
        self.subscribers.write().unwrap().clear();
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
        store.update("counter", |v| {
            serde_json::json!(v.as_i64().expect("test: counter should be i64") + 1)
        });
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

        let _sub = store.subscribe(move |key, value| {
            if let Ok(mut notifications) = notifications_clone.lock() {
                notifications.push((key.to_string(), value.clone()));
            }
        });
        store.set("key1", serde_json::json!("value1"));
        store.set("key2", serde_json::json!("value2"));

        let notifications = notifications
            .lock()
            .expect("test: mutex should not be poisoned");
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

    #[test]
    fn test_signal_store_creation() {
        let store = SignalStore::new();
        assert_eq!(store.get("nonexistent"), None);
        assert!(store.keys().is_empty());
    }

    #[test]
    fn test_signal_store_update_with_json_objects() {
        let store = SignalStore::new();
        let initial = serde_json::json!({"name": "test", "count": 0});
        store.set("data", initial);

        store.update("data", |v| {
            let mut obj = v.as_object().unwrap().clone();
            obj.insert("count".to_string(), serde_json::json!(1));
            serde_json::json!(obj)
        });

        let result = store.get("data").unwrap();
        assert_eq!(result["count"], serde_json::json!(1));
    }

    #[test]
    fn test_signal_store_with_json_arrays() {
        let store = SignalStore::new();
        store.set("items", serde_json::json!([1, 2, 3]));
        let items = store.get("items").unwrap();
        assert!(items.is_array());
        assert_eq!(items.as_array().unwrap().len(), 3);

        store.update("items", |v| {
            let mut arr = v.as_array().unwrap().clone();
            arr.push(serde_json::json!(4));
            serde_json::json!(arr)
        });

        let updated = store.get("items").unwrap();
        assert_eq!(updated.as_array().unwrap().len(), 4);
    }

    #[test]
    fn test_signal_store_multiple_signals() {
        let store = SignalStore::new();
        store.set("string", serde_json::json!("hello"));
        store.set("number", serde_json::json!(42));
        store.set("bool", serde_json::json!(true));
        store.set("null", serde_json::json!(null));

        assert_eq!(store.get("string"), Some(serde_json::json!("hello")));
        assert_eq!(store.get("number"), Some(serde_json::json!(42)));
        assert_eq!(store.get("bool"), Some(serde_json::json!(true)));
        assert_eq!(store.get("null"), Some(serde_json::json!(null)));
        assert_eq!(store.keys().len(), 4);
    }

    #[test]
    fn test_signal_store_nested_json() {
        let store = SignalStore::new();
        let nested = serde_json::json!({
          "user": {
            "profile": {
              "name": "Alice",
              "settings": {"theme": "dark"}
            },
            "tags": ["admin", "active"]
          }
        });
        store.set("config", nested);

        let retrieved = store.get("config").unwrap();
        assert_eq!(
            retrieved["user"]["profile"]["name"],
            serde_json::json!("Alice")
        );
        assert_eq!(retrieved["user"]["tags"][0], serde_json::json!("admin"));
    }

    #[test]
    fn test_signal_store_subscriber_receives_updates() {
        use std::sync::{Arc, Mutex};

        let store = SignalStore::new();
        let received: Arc<Mutex<Vec<serde_json::Value>>> = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let _sub = store.subscribe(move |_key, value| {
            if let Ok(mut r) = received_clone.lock() {
                r.push(value.clone());
            }
        });

        store.set("a", serde_json::json!(1));
        store.set("b", serde_json::json!(2));
        store.set("c", serde_json::json!(3));

        let r = received.lock().expect("mutex should not be poisoned");
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], serde_json::json!(1));
        assert_eq!(r[1], serde_json::json!(2));
        assert_eq!(r[2], serde_json::json!(3));
    }

    #[test]
    fn test_signal_store_delete_notifies() {
        use std::sync::{Arc, Mutex};

        let store = SignalStore::new();
        let delete_called: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let delete_called_clone = delete_called.clone();

        let _sub = store.subscribe(move |key, _value| {
            if key == "todelete" {
                if let Ok(mut called) = delete_called_clone.lock() {
                    *called = true;
                }
            }
        });

        // First call: set triggers notification
        store.set("todelete", serde_json::json!("value"));
        {
            let called = delete_called.lock().expect("mutex should not be poisoned");
            assert!(*called, "set should trigger subscriber for matching key");
        }
        // Reset flag then verify delete does NOT trigger notification
        {
            let mut called = delete_called.lock().expect("mutex should not be poisoned");
            *called = false;
        }
        store.delete("todelete");
        let called = delete_called.lock().expect("mutex should not be poisoned");
        assert!(
            !*called,
            "delete should not trigger subscriber notification"
        );
    }

    #[test]
    fn test_signal_store_to_json_from_json() {
        let store = SignalStore::new();
        store.set("a", serde_json::json!(1));
        store.set("b", serde_json::json!({"nested": true}));

        let json = store.to_json();
        assert_eq!(json["a"], serde_json::json!(1));
        assert_eq!(json["b"]["nested"], serde_json::json!(true));

        let new_store = SignalStore::new();
        new_store.from_json(json.clone());
        assert_eq!(new_store.get("a"), Some(serde_json::json!(1)));
        assert_eq!(
            new_store.get("b"),
            Some(serde_json::json!({"nested": true}))
        );
    }

    #[test]
    fn test_signal_store_update_nonexistent_key() {
        let store = SignalStore::new();
        store.update("nonexistent", |v| {
            serde_json::json!(v.as_i64().unwrap_or(0) + 1)
        });
        assert_eq!(store.get("nonexistent"), None);
    }
}
