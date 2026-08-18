//! Result type alias using AppError.

pub type Result<T> = std::result::Result<T, crate::error::AppError>;
