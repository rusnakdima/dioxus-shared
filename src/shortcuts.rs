//! Global keyboard shortcuts registry
//!
//! Allows registering shortcuts from schema at runtime.
//! Shortcuts are stored globally and can be triggered via keyboard events.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Global shortcuts registry
static SHORTCUTS: Lazy<Mutex<HashMap<String, ShortcutDef>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Shortcut definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutDef {
    pub id: String,
    pub keys: String,
    pub action: String,
}

impl ShortcutDef {
    pub fn new(id: &str, keys: &str, action: &str) -> Self {
        Self {
            id: id.to_string(),
            keys: keys.to_string(),
            action: action.to_string(),
        }
    }
}

/// Register a shortcut from schema
pub fn register_shortcut(id: &str, keys: &str, action: &str) {
    let shortcut = ShortcutDef::new(id, keys, action);
    match SHORTCUTS.lock() {
        Ok(mut guard) => guard.insert(id.to_string(), shortcut),
        Err(poisoned) => poisoned.into_inner().insert(id.to_string(), shortcut),
    };
}

/// Register multiple shortcuts from a slice
pub fn register_shortcuts(shortcuts: &[ShortcutDef]) {
    let mut registry = match SHORTCUTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for shortcut in shortcuts {
        registry.insert(shortcut.id.clone(), shortcut.clone());
    }
}

/// Get all registered shortcuts
pub fn get_all_shortcuts() -> Vec<ShortcutDef> {
    match SHORTCUTS.lock() {
        Ok(guard) => guard.values().cloned().collect(),
        Err(poisoned) => poisoned.into_inner().values().cloned().collect(),
    }
}

/// Find a shortcut by its key combination
pub fn find_shortcut_by_keys(keys: &str) -> Option<ShortcutDef> {
    let registry = match SHORTCUTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    registry.values().find(|s| s.keys == keys).cloned()
}

/// Find a shortcut by its action name
pub fn find_shortcut_by_action(action: &str) -> Option<ShortcutDef> {
    let registry = match SHORTCUTS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    registry.values().find(|s| s.action == action).cloned()
}

/// Clear all registered shortcuts
pub fn clear_shortcuts() {
    match SHORTCUTS.lock() {
        Ok(mut guard) => guard.clear(),
        Err(poisoned) => poisoned.into_inner().clear(),
    }
}

/// Parse key string to normalized form (e.g., "ctrl+enter" -> "ctrl+enter")
pub fn normalize_keys(keys: &str) -> String {
    keys.to_lowercase()
        .replace("control", "ctrl")
        .replace("cmd", "ctrl")
        .replace("command", "ctrl")
        .replace("option", "alt")
        .replace(" ", "")
}

/// Check if a key event matches a shortcut pattern
pub fn match_keys(event_keys: &str, shortcut_keys: &str) -> bool {
    normalize_keys(event_keys) == normalize_keys(shortcut_keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        clear_shortcuts();
    }

    fn teardown() {
        clear_shortcuts();
    }

    #[test]
    fn test_normalize_keys_ctrl_s() {
        setup();
        let result = normalize_keys("ctrl+s");
        assert_eq!(result, "ctrl+s");
        teardown();
    }

    #[test]
    fn test_normalize_keys_cmdorctrl_space() {
        setup();
        let result = normalize_keys("CmdOrCtrl+Space");
        // "CmdOrCtrl+Space" → "cmdorctrl+space" (lowercase)
        // → replace("cmd","ctrl"): "ctrlorctrl+space"
        // → replace("command","ctrl"): no match
        // → replace("option","alt"): no match
        // → replace(" ", ""): no match (already no space)
        assert_eq!(result, "ctrlorctrl+space");
        teardown();
    }

    #[test]
    fn test_match_keys_exact() {
        setup();
        let result = match_keys("Ctrl+S", "Ctrl+S");
        assert!(result);
        teardown();
    }

    #[test]
    fn test_match_keys_multiple() {
        setup();
        // Should match when event matches any of the provided shortcut keys
        let result = match_keys("Ctrl+S", "Ctrl+W");
        assert!(!result); // "ctrl+s" != "ctrl+w"
        let result2 = match_keys("Ctrl+S", "Ctrl+S");
        assert!(result2);
        teardown();
    }

    #[test]
    fn test_register_and_get_all_shortcuts() {
        setup();
        register_shortcut("quit", "Ctrl+Q", "quit");
        let shortcuts = get_all_shortcuts();
        assert!(shortcuts
            .iter()
            .any(|s| s.keys == "Ctrl+Q" && s.action == "quit"));
        teardown();
    }

    #[test]
    fn test_clear_shortcuts() {
        setup();
        register_shortcut("quit", "Ctrl+Q", "quit");
        let before = get_all_shortcuts();
        assert!(!before.is_empty());
        clear_shortcuts();
        let after = get_all_shortcuts();
        assert!(after.is_empty());
        teardown();
    }

    #[test]
    fn test_register_shortcut_with_key_and_action() {
        setup();
        register_shortcut("save", "Ctrl+S", "save_document");
        let shortcuts = get_all_shortcuts();
        assert_eq!(shortcuts.len(), 1);
        assert_eq!(shortcuts[0].id, "save");
        assert_eq!(shortcuts[0].keys, "Ctrl+S");
        assert_eq!(shortcuts[0].action, "save_document");
        teardown();
    }

    #[test]
    fn test_lookup_shortcut_by_keys() {
        setup();
        register_shortcut("quit", "Ctrl+Q", "quit_action");
        register_shortcut("save", "Ctrl+S", "save_action");
        let found = find_shortcut_by_keys("Ctrl+Q");
        assert!(found.is_some());
        let shortcut = found.unwrap();
        assert_eq!(shortcut.id, "quit");
        assert_eq!(shortcut.action, "quit_action");
        // Lookup non-existent
        let not_found = find_shortcut_by_keys("Ctrl+Z");
        assert!(not_found.is_none());
        teardown();
    }

    #[test]
    fn test_lookup_shortcut_by_action() {
        setup();
        register_shortcut("quit", "Ctrl+Q", "quit_action");
        register_shortcut("save", "Ctrl+S", "save_action");
        let found = find_shortcut_by_action("save_action");
        assert!(found.is_some());
        let shortcut = found.unwrap();
        assert_eq!(shortcut.id, "save");
        assert_eq!(shortcut.keys, "Ctrl+S");
        // Lookup non-existent action
        let not_found = find_shortcut_by_action("nonexistent");
        assert!(not_found.is_none());
        teardown();
    }

    #[test]
    fn test_get_all_shortcuts() {
        setup();
        register_shortcut("quit", "Ctrl+Q", "quit");
        register_shortcut("save", "Ctrl+S", "save");
        register_shortcut("copy", "Ctrl+C", "copy");
        let shortcuts = get_all_shortcuts();
        assert_eq!(shortcuts.len(), 3);
        teardown();
    }

    #[test]
    fn test_shortcut_with_modifiers_ctrl_alt_shift() {
        setup();
        // Ctrl + modifier
        register_shortcut("copy", "Ctrl+C", "copy");
        let found = find_shortcut_by_keys("Ctrl+C");
        assert!(found.is_some());

        // Alt + modifier
        register_shortcut("alt_tab", "Alt+Tab", "switch_window");
        let found_alt = find_shortcut_by_keys("Alt+Tab");
        assert!(found_alt.is_some());
        assert_eq!(found_alt.unwrap().action, "switch_window");

        // Shift + modifier
        register_shortcut("redo", "Shift+Ctrl+Z", "redo");
        let found_shift = find_shortcut_by_keys("Shift+Ctrl+Z");
        assert!(found_shift.is_some());
        assert_eq!(found_shift.unwrap().action, "redo");

        // Test normalize_keys with modifiers
        assert_eq!(normalize_keys("CTRL+C"), "ctrl+c");
        assert_eq!(normalize_keys("ALT+TAB"), "alt+tab");
        assert_eq!(normalize_keys("SHIFT+CTRL+Z"), "shift+ctrl+z");
        teardown();
    }

    #[test]
    fn test_register_and_find_multiple_shortcuts() {
        setup();
        register_shortcuts(&[
            ShortcutDef::new("new", "Ctrl+N", "new_file"),
            ShortcutDef::new("open", "Ctrl+O", "open_file"),
            ShortcutDef::new("close", "Ctrl+W", "close_tab"),
        ]);
        let all = get_all_shortcuts();
        assert_eq!(all.len(), 3);
        assert!(find_shortcut_by_keys("Ctrl+N").is_some());
        assert!(find_shortcut_by_keys("Ctrl+O").is_some());
        assert!(find_shortcut_by_keys("Ctrl+W").is_some());
        teardown();
    }
}
