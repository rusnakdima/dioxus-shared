//! Schema types for SDUI (Schema-Driven UI).
//!
//! This module contains all types needed for schema-driven UI rendering.

pub mod app;
pub mod component;
pub mod data_binding;
pub mod element_layout;
pub mod handlers;
pub mod layout;
pub mod page;
pub mod theme;
pub mod ui_schema;

pub use app::*;
pub use component::*;
pub use data_binding::*;
pub use element_layout::*;
pub use handlers::*;
pub use layout::*;
pub use page::*;
pub use theme::*;
pub use ui_schema::UiSchema;
pub use page::Schema;
