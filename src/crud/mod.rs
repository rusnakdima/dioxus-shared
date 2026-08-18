//! CRUD module for database operations.

pub mod cascade_service;
pub mod service;
pub mod types;

pub use cascade_service::CascadeService;
pub use service::CrudService;
pub use types::{CrudFilter, CrudQuery, CrudResult, PaginatedResult};
