//! Entities module
//!
//! Core domain entities used across all Dioxus applications.
//! These are plain data structures with no business logic.
//!
//! # App-level entity registration
//!
//! Each Dioxus application owns its domain entities. At startup the app
//! creates an [`EntityRegistry`], registers its app entities through
//! [`EntityRegistry::register`] with schemas built by
//! [`EntitySchema::new`], then hands the registry to services that need to
//! validate or query dynamic entities.
//!
//! ```rust,ignore
//! let registry = EntityRegistry::new();
//! registry.register("task", EntitySchema::new(
//!     "task",
//!     "tasks",
//!     json!({ "type": "object", "properties": { "title": { "type": "string" } } }),
//! ));
//! // hand `registry` to services
//! ```

pub mod user;
pub mod session;
pub mod config;
pub mod registry;

pub use user::{User, UserRole};
pub use session::Session;
pub use config::AppConfig;
pub use registry::{EntitySchema, EntityRegistry};
