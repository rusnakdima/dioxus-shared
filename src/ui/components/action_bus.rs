//! Action Bus — event dispatch and state binding for SDUI
//!
//! Provides a shared context for:
//! - Dispatching named actions from interactive schema elements
//! - Binding form field values to shared state
//! - Page navigation
//! - Modal dialog management

use std::collections::{HashMap, VecDeque};
use dioxus::prelude::*;

use crate::themes::ThemeMode;

// Compatibility re-export so existing consumers importing the local enum keep
// compiling. The canonical type lives at `crate::themes::ThemeMode`.
pub use crate::themes::ThemeMode as _ActionBusThemeModeReExport;

/// An action dispatched from an interactive schema element
#[derive(Debug, Clone, PartialEq)]
pub struct AppAction {
    pub name: String,
    pub source: String,
    pub payload: Option<serde_json::Value>,
}

/// Navigation action
#[derive(Debug, Clone, PartialEq)]
pub struct NavigateAction {
    pub route: String,
    pub params: Option<HashMap<String, String>>,
}

/// ActionBus context type — holds all SDUI state
/// Provide this at the app root via `provide_context`.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionBus {
    pub dispatch: Signal<VecDeque<AppAction>>,
    pub bindings: Signal<HashMap<String, String>>,
    pub navigate: Signal<Option<NavigateAction>>,
    pub current_route: Signal<String>,
    pub current_modal: Signal<Option<String>>,
    pub theme_mode: Signal<ThemeMode>,
}

impl ActionBus {
    pub fn new(initial_route: &str) -> Self {
        Self {
            dispatch: Signal::new(VecDeque::new()),
            bindings: Signal::new(HashMap::new()),
            navigate: Signal::new(None),
            current_route: Signal::new(initial_route.to_string()),
            current_modal: Signal::new(None),
            theme_mode: Signal::new(ThemeMode::System),
        }
    }

    /// Dispatch a named action from an interactive element
    pub fn dispatch(&mut self, action: AppAction) {
        self.dispatch.write().push_back(action);
    }

    /// Set a bound form field value
    pub fn set_binding(&mut self, key: &str, value: &str) {
        self.bindings.write().insert(key.to_string(), value.to_string());
    }

    /// Get a bound form field value
    pub fn get_binding(&self, key: &str) -> Option<String> {
        self.bindings.read().get(key).cloned()
    }

    /// Request navigation to a different route
    pub fn navigate(&mut self, route: &str, params: Option<HashMap<String, String>>) {
        self.navigate.set(Some(NavigateAction {
            route: route.to_string(),
            params,
        }));
    }

    /// Open a modal by ID
    pub fn open_modal(&mut self, modal_id: &str) {
        self.current_modal.set(Some(modal_id.to_string()));
    }

    /// Close the current modal
    pub fn close_modal(&mut self) {
        self.current_modal.set(None);
    }

    /// Toggle between light and dark theme.
    ///
    /// Cycles Light ↔ Dark. If the current mode is `System`, first transition
    /// to the opposite of the resolved light/dark state.
    pub fn toggle_theme(&mut self) {
        let current = *self.theme_mode.read();
        let next = match current {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::System => {
                if self.is_dark_mode() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                }
            }
        };
        self.theme_mode.set(next);
    }

    /// Pop the next pending action from the queue
    /// Returns None if queue is empty
    pub fn pop_action(&mut self) -> Option<AppAction> {
        self.dispatch.write().pop_front()
    }

    /// Clear all pending actions
    pub fn clear_actions(&mut self) {
        self.dispatch.write().clear();
    }

    /// Clear a specific binding
    pub fn clear_binding(&mut self, key: &str) {
        self.bindings.write().remove(key);
    }

    /// Check if a binding exists and is non-empty
    pub fn has_value(&self, key: &str) -> bool {
        self.get_binding(key).map(|v| !v.is_empty()).unwrap_or(false)
    }

    /// Get current route value
    pub fn current_route(&self) -> String {
        self.current_route.read().clone()
    }

    /// Get current modal ID
    pub fn current_modal(&self) -> Option<String> {
        self.current_modal.read().clone()
    }

    /// Get navigate pending action (returns and clears)
    pub fn pop_navigate(&mut self) -> Option<NavigateAction> {
        self.navigate.write().take()
    }

    /// Resolve the current mode to a concrete light/dark value.
    ///
    /// `System` is treated as Light until real platform detection is wired in
    /// `ThemeProvider`. Callers that need a definitive answer should use this.
    pub fn is_dark_mode(&self) -> bool {
        match *self.theme_mode.read() {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => false,
        }
    }
}
