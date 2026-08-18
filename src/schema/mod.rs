//! Schema types for SDUI (Schema-Driven UI).
//!
//! This module contains all types needed for schema-driven UI rendering.

pub mod app;
pub mod builder;
pub mod component;
pub mod converter;
pub mod data_binding;
pub mod element_layout;
pub mod handlers;
pub mod layout;
pub mod page;
pub mod semantic_types;
pub mod theme;
pub mod ui_schema;
pub mod validator;
pub mod variant;

pub use app::*;
pub use builder::{element, modal, page, shortcut, SchemaBuilder};
pub use component::*;
pub use converter::convert_tauri_schema;
pub use data_binding::*;
pub use element_layout::*;
pub use handlers::*;
pub use layout::*;
pub use page::Schema;
pub use page::*;
pub use theme::*;
pub use ui_schema::UiSchema;
pub use validator::{
    validate_element, validate_element_ids, validate_page_routes, validate_schema, ValidationError,
    ValidationResult,
};
