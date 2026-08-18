//! Entities module
//!
//! Core domain entities used across ALL Dioxus applications.
//! These are PLAIN DATA STRUCTURES with no business logic.
//!
//! # Shared types (kept here):
//! - Session: User session across apps
//! - User: User account
//! - AppConfig: Application configuration
//! - EntitySchema / EntityRegistry: Dynamic entity registry for schema validation
//!
//! # App-specific types
//! All app-specific entities have been moved to their respective apps' src/domain/entities/

pub mod config;
pub mod registry;
pub mod session;
pub mod user;

pub use config::AppConfig;
pub use registry::{EntityRegistry, EntitySchema};
pub use session::Session;
pub use user::{User, UserRole};
