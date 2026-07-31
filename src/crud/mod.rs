//! CRUD module for database operations.

pub mod service;
pub mod types;

pub use types::{CrudFilter, CrudQuery, CrudResult, PaginatedResult};
pub use service::CrudService;
