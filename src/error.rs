//! Error types for the application.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    Duplicate(String),
    Unauthorized,
    Forbidden,
    Internal(String),
    Database(String),
    Network(String),
    Io(String),
    PermissionDenied(String),
    InvalidPath(String),
    RequestFailed(String),
    Lock(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(entity) => write!(f, "{} not found", entity),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Duplicate(entity) => write!(f, "{} already exists", entity),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Io(msg) => write!(f, "IO error: {}", msg),
            AppError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            AppError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            AppError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            AppError::Lock(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

#[cfg(feature = "dioxus-desktop")]
impl From<tokio_tungstenite::tungstenite::Error> for AppError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<nosql_orm::error::OrmError> for AppError {
    fn from(err: nosql_orm::error::OrmError) -> Self {
        match err {
            nosql_orm::error::OrmError::NotFound(entity) => AppError::NotFound(entity),
            nosql_orm::error::OrmError::Validation(msg) => AppError::ValidationError(msg),
            nosql_orm::error::OrmError::Duplicate(entity) => AppError::Duplicate(entity),
            nosql_orm::error::OrmError::Connection(msg) => AppError::Database(msg),
            nosql_orm::error::OrmError::Provider(msg) => AppError::Database(msg),
            nosql_orm::error::OrmError::Query(msg) => AppError::Database(msg),
            nosql_orm::error::OrmError::Internal(msg) => AppError::Internal(msg),
            _ => AppError::Internal(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> for AppError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, T>>) -> Self {
        AppError::Lock("mutex poisoned".to_string())
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> for AppError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>) -> Self {
        AppError::Lock("RwLock write guard poisoned".to_string())
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>> for AppError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>) -> Self {
        AppError::Lock("RwLock read guard poisoned".to_string())
    }
}

impl AppError {
    pub fn into_response<T>(self) -> crate::response::Response<T> {
        use crate::response::{Response, Status};
        match self {
            AppError::NotFound(entity) => Response {
                status: Status::NotFound,
                message: format!("{} not found", entity),
                data: None,
            },
            AppError::ValidationError(msg) => Response {
                status: Status::ValidationError,
                message: msg,
                data: None,
            },
            AppError::Duplicate(entity) => Response {
                status: Status::Duplicate,
                message: entity,
                data: None,
            },
            AppError::Unauthorized => Response {
                status: Status::Unauthorized,
                message: "Unauthorized".into(),
                data: None,
            },
            AppError::Forbidden => Response {
                status: Status::Forbidden,
                message: "Forbidden".into(),
                data: None,
            },
            AppError::Internal(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::Database(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::Network(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::Io(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::PermissionDenied(msg) => Response {
                status: Status::Forbidden,
                message: msg,
                data: None,
            },
            AppError::InvalidPath(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::RequestFailed(msg) => Response {
                status: Status::Error,
                message: msg,
                data: None,
            },
            AppError::Lock(msg) => Response {
                status: Status::Error,
                message: format!("Lock error: {}", msg),
                data: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display_not_found() {
        let err = AppError::NotFound("User".to_string());
        assert_eq!(format!("{}", err), "User not found");
    }

    #[test]
    fn test_app_error_display_validation_error() {
        let err = AppError::ValidationError("Invalid email".to_string());
        assert_eq!(format!("{}", err), "Validation error: Invalid email");
    }

    #[test]
    fn test_app_error_display_internal() {
        let err = AppError::Internal("Unexpected error".to_string());
        assert_eq!(format!("{}", err), "Internal error: Unexpected error");
    }

    // -- From impls ----------------------------------------------------------------

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let app_err = AppError::from(json_err);
        match app_err {
            AppError::ValidationError(_) => {}
            other => panic!("expected ValidationError, got {:?}", other),
        }
    }

    #[test]
    fn test_from_io_error() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_err = AppError::from(io_err);
        match app_err {
            AppError::Io(msg) => assert!(msg.contains("file not found")),
            other => panic!("expected Io, got {:?}", other),
        }
    }

    #[cfg(feature = "dioxus-desktop")]
    #[test]
    fn test_from_tungstenite_error() {
        use tokio_tungstenite::tungstenite::Error;
        // Use a simple constructor that doesn't require a connection
        let err = Error::ConnectionClosed;
        let app_err = AppError::from(err);
        match app_err {
            AppError::Network(msg) => {
                assert!(msg.contains("Connection closed") || msg.contains("closed"))
            }
            other => panic!("expected Network, got {:?}", other),
        }
    }

    #[test]
    fn test_from_orm_error() {
        use nosql_orm::error::OrmError;
        // NotFound variant
        let orm_err = OrmError::NotFound("User".to_string());
        let app_err = AppError::from(orm_err);
        match app_err {
            AppError::NotFound(entity) => assert_eq!(entity, "User"),
            other => panic!("expected NotFound, got {:?}", other),
        }
        // Validation variant
        let orm_err = OrmError::Validation("bad input".to_string());
        let app_err = AppError::from(orm_err);
        match app_err {
            AppError::ValidationError(msg) => assert!(msg.contains("bad input")),
            other => panic!("expected ValidationError, got {:?}", other),
        }
        // Connection variant
        let orm_err = OrmError::Connection("db down".to_string());
        let app_err = AppError::from(orm_err);
        match app_err {
            AppError::Database(msg) => assert!(msg.contains("db down")),
            other => panic!("expected Database, got {:?}", other),
        }
    }

    // -- into_response -------------------------------------------------------------

    #[test]
    fn test_into_response_not_found() {
        let err = AppError::NotFound("User".to_string());
        let resp = err.into_response::<()>();
        assert_eq!(resp.status, crate::response::Status::NotFound);
        assert!(resp.message.contains("User"));
        assert!(resp.data.is_none());
    }

    #[test]
    fn test_into_response_validation_error() {
        let err = AppError::ValidationError("invalid email".to_string());
        let resp = err.into_response::<()>();
        assert_eq!(resp.status, crate::response::Status::ValidationError);
        assert!(resp.message.contains("invalid email"));
    }

    #[test]
    fn test_into_response_internal() {
        let err = AppError::Internal("oops".to_string());
        let resp = err.into_response::<()>();
        assert_eq!(resp.status, crate::response::Status::Error);
        assert!(resp.message.contains("oops"));
    }

    #[test]
    fn test_into_response_unauthorized() {
        let err = AppError::Unauthorized;
        let resp = err.into_response::<()>();
        assert_eq!(resp.status, crate::response::Status::Unauthorized);
        assert!(resp.message.contains("Unauthorized"));
    }

    #[test]
    fn test_into_response_forbidden() {
        let err = AppError::Forbidden;
        let resp = err.into_response::<()>();
        assert_eq!(resp.status, crate::response::Status::Forbidden);
    }

    // -- serde round-trip --------------------------------------------------------

    #[test]
    fn test_app_error_serde_roundtrip_not_found() {
        let err = AppError::NotFound("User".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::NotFound(a), AppError::NotFound(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_validation_error() {
        let err = AppError::ValidationError("Invalid input".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::ValidationError(a), AppError::ValidationError(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_duplicate() {
        let err = AppError::Duplicate("Order".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Duplicate(a), AppError::Duplicate(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_unauthorized() {
        let err = AppError::Unauthorized;
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Unauthorized, AppError::Unauthorized) => {}
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_forbidden() {
        let err = AppError::Forbidden;
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Forbidden, AppError::Forbidden) => {}
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_internal() {
        let err = AppError::Internal("Unexpected failure".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Internal(a), AppError::Internal(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_database() {
        let err = AppError::Database("Connection refused".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Database(a), AppError::Database(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_network() {
        let err = AppError::Network("timeout".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Network(a), AppError::Network(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_io() {
        let err = AppError::Io("file not found".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Io(a), AppError::Io(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_permission_denied() {
        let err = AppError::PermissionDenied("access denied".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::PermissionDenied(a), AppError::PermissionDenied(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_invalid_path() {
        let err = AppError::InvalidPath("/invalid//path".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::InvalidPath(a), AppError::InvalidPath(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_request_failed() {
        let err = AppError::RequestFailed("Request timeout".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::RequestFailed(a), AppError::RequestFailed(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    #[test]
    fn test_app_error_serde_roundtrip_lock() {
        let err = AppError::Lock("mutex poisoned".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let decoded: AppError = serde_json::from_str(&json).unwrap();
        match (&err, &decoded) {
            (AppError::Lock(a), AppError::Lock(b)) => assert_eq!(a, b),
            _ => panic!("variant mismatch"),
        }
    }

    // -- serde JSON format --------------------------------------------------------

    #[test]
    fn test_app_error_serde_camel_case() {
        let err = AppError::NotFound("User".to_string());
        let json = serde_json::to_string(&err).unwrap();
        // Must use camelCase (camelCase -> notFound)
        assert!(json.contains("notFound"));
        assert!(!json.contains("NotFound"));
    }
}
