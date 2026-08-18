//! Services module
//!
//! Reusable business logic services initialized from environment + entities.

pub mod auth;
pub mod base_crud_service;

pub use auth::{global, AuthService};
pub use base_crud_service::BaseCrudService;
