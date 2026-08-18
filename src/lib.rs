//! `dioxus-shared` - Shared Rust library for Dioxus applications
//!
//! This library provides pure Rust business logic for Dioxus applications,
//! including algorithms, CRUD operations, schema management, RBAC, and more.
//! All modules are framework-agnostic and can be used directly without IPC.
//!
//! # Core Modules
//!
//! - [`algorithms`] - Sorting, search, graph, and sanitization algorithms
//! - [`crud`] - CRUD service for database operations
//! - [`storage`] - SignalStore and JsonProvider integration
//! - [`schema`] - SDUI schema types
//! - [`rbac`] - Authentication and role-based access control
//! - [`logger`] - Structured logging
//! - [`env`] - Environment configuration
//! - [`error`] - Error types
//! - [`response`] - Response wrapper types
//! - [`update`] - Update checking and installation
//!
//! # Example
//!
//! ```rust,no_run
//! use dioxus_shared::algorithms::{bubble_sort, AlgorithmRegistry};
//! use dioxus_shared::env::EnvConfig;
//! use dioxus_shared::storage::SignalStore;
//! ```

pub mod algorithms;
pub mod crud;
pub mod entities;
pub mod env;
pub mod error;
pub mod logger;
pub mod mcp;
pub mod rbac;
pub mod response;
pub mod result;
pub mod schema;
pub mod services;
pub mod storage;
pub mod update;

// UI components (requires dioxus-ui feature)
#[cfg(feature = "dioxus-ui")]
pub mod ui;

// Shortcuts module for global keyboard shortcut registration
pub mod shortcuts;

// Theme system
pub mod themes;

// Schema-driven UI engine (requires dioxus-ui feature)
#[cfg(feature = "dioxus-ui")]
pub mod ui_engine;

// UniChat KAS handlers (for desktop applications)
pub mod unichat;

/// Get the theme CSS for embedding in Dioxus document
///
/// This returns the TailwindCSS v4 theme CSS that provides:
/// - Utility classes for all library components
/// - Design tokens via @theme directive
///
/// Usage in Dioxus app:
/// ```rust,ignore
/// rsx! {
///     style { {dioxus_shared::get_theme_css()} }
///     DynamicPage { page: schema }
/// }
/// ```
pub fn get_theme_css() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets/theme.css")
}

// Re-export commonly used types
pub use algorithms::{algo_execute, Algorithm, AlgorithmInput, AlgorithmOutput, AlgorithmRegistry};
pub use crud::{CrudFilter, CrudQuery, CrudResult, CrudService, PaginatedResult};
pub use entities::{EntityRegistry, EntitySchema};
pub use env::EnvConfig;
pub use error::AppError;
pub use logger::{LogEntry, LogLevel, Logger};
pub use mcp::{JsonRpcError, JsonRpcId, JsonRpcRequest, JsonRpcResponse};
pub use rbac::{
    get_current_user, login, logout, rbac_assign_role_to_user, rbac_create_permission,
    rbac_create_role, rbac_delete_permission, rbac_delete_role, rbac_get_role_permissions,
    rbac_get_user_roles, rbac_grant_permission, rbac_list_permissions, rbac_list_roles,
    rbac_remove_role_from_user, rbac_revoke_permission, register, Permission, Role, RolePermission,
    Session, User, UserRole,
};
pub use response::{Response, Status};
pub use result::Result;
pub use schema::{AppConfig, Component, Layout, Modal, Page, Schema, Shortcut, UiSchema};
pub use services::BaseCrudService;
pub use shortcuts::{
    clear_shortcuts, find_shortcut_by_action, find_shortcut_by_keys, get_all_shortcuts, match_keys,
    normalize_keys, register_shortcut, register_shortcuts,
};
pub use storage::{
    create_json_provider, create_json_provider_with_config, setup_schema_system, JsonProvider,
    JsonProviderState, SchemaConfig, SchemaSyncService, SchemaSyncState, SchemaSystem, SignalStore,
};
