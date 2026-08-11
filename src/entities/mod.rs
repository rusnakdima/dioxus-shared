//! Entities module
//!
//! Core domain entities used across all Dioxus applications.
//! These are plain data structures with no business logic.

pub mod user;
pub mod session;
pub mod config;
pub mod registry;

pub use user::{User, UserRole};
pub use session::Session;
pub use config::AppConfig;
pub use registry::{EntitySchema, EntityRegistry};
