//! Response types for CRUD operations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
  Success,
  Created,
  Updated,
  Deleted,
  Error,
  ValidationError,
  NotFound,
  Unauthorized,
  Forbidden,
  Info,
  Warning,
  Duplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
  pub status: Status,
  pub message: String,
  pub data: Option<T>,
}

impl<T> Response<T> {
  pub fn success(data: T, message: Option<&str>) -> Self {
    Self {
      status: Status::Success,
      message: message.map(String::from).unwrap_or_default(),
      data: Some(data),
    }
  }

  pub fn created(data: T) -> Self {
    Self {
      status: Status::Created,
      message: String::new(),
      data: Some(data),
    }
  }

  pub fn updated(data: T) -> Self {
    Self {
      status: Status::Updated,
      message: String::new(),
      data: Some(data),
    }
  }

  pub fn deleted(data: T) -> Self {
    Self {
      status: Status::Deleted,
      message: String::new(),
      data: Some(data),
    }
  }

  pub fn error(message: impl Into<String>) -> Self {
    Self {
      status: Status::Error,
      message: message.into(),
      data: None,
    }
  }

  pub fn error_with_data(data: T, message: impl Into<String>) -> Self {
    Self {
      status: Status::Error,
      message: message.into(),
      data: Some(data),
    }
  }

  pub fn validation_error(message: impl Into<String>) -> Self {
    Self {
      status: Status::ValidationError,
      message: message.into(),
      data: None,
    }
  }

  pub fn not_found(entity: impl Into<String>) -> Self {
    Self {
      status: Status::NotFound,
      message: format!("{} not found", entity.into()),
      data: None,
    }
  }

  pub fn unauthorized(message: impl Into<String>) -> Self {
    Self {
      status: Status::Unauthorized,
      message: message.into(),
      data: None,
    }
  }

  pub fn forbidden(message: impl Into<String>) -> Self {
    Self {
      status: Status::Forbidden,
      message: message.into(),
      data: None,
    }
  }

  pub fn error_with_status(status: Status, message: impl Into<String>) -> Self {
    Self {
      status,
      message: message.into(),
      data: None,
    }
  }

  pub fn err(msg: impl Into<String>) -> Self {
    Self {
      status: Status::Error,
      message: msg.into(),
      data: None,
    }
  }
}

impl<T: Clone> Response<T> {
  pub fn map_data<U: Clone>(self, f: impl FnOnce(T) -> U) -> Response<U> {
    Response {
      status: self.status,
      message: self.message,
      data: self.data.map(f),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_response_success() {
    let resp = Response::success("data", Some("success message"));
    assert_eq!(resp.status, Status::Success);
    assert_eq!(resp.message, "success message");
    assert_eq!(resp.data, Some("data"));
  }

  #[test]
  fn test_response_not_found() {
    let resp: Response<()> = Response::not_found("User");
    assert_eq!(resp.status, Status::NotFound);
    assert_eq!(resp.message, "User not found");
  }

  #[test]
  fn test_response_error() {
    let resp: Response<()> = Response::error("Something went wrong");
    assert_eq!(resp.status, Status::Error);
    assert_eq!(resp.message, "Something went wrong");
  }

  // -- Additional constructor tests ---------------------------------------------

  #[test]
  fn test_response_created() {
    let resp = Response::created("new_item");
    assert_eq!(resp.status, Status::Created);
    assert!(resp.message.is_empty());
    assert_eq!(resp.data, Some("new_item"));
  }

  #[test]
  fn test_response_updated() {
    let resp = Response::updated("updated_item");
    assert_eq!(resp.status, Status::Updated);
    assert!(resp.message.is_empty());
    assert_eq!(resp.data, Some("updated_item"));
  }

  #[test]
  fn test_response_deleted() {
    let resp = Response::deleted("deleted_item");
    assert_eq!(resp.status, Status::Deleted);
    assert!(resp.message.is_empty());
    assert_eq!(resp.data, Some("deleted_item"));
  }

  #[test]
  fn test_response_error_with_message() {
    let resp: Response<()> = Response::error("error message");
    assert_eq!(resp.status, Status::Error);
    assert_eq!(resp.message, "error message");
    assert!(resp.data.is_none());
  }

  #[test]
  fn test_response_error_with_data() {
    let resp = Response::error_with_data("some_data", "error with data");
    assert_eq!(resp.status, Status::Error);
    assert_eq!(resp.message, "error with data");
    assert_eq!(resp.data, Some("some_data"));
  }

  #[test]
  fn test_response_not_found_entity() {
    let resp: Response<()> = Response::not_found("Widget");
    assert_eq!(resp.status, Status::NotFound);
    assert_eq!(resp.message, "Widget not found");
    assert!(resp.data.is_none());
  }

  #[test]
  fn test_response_invalid() {
    let resp: Response<()> = Response::validation_error("invalid input");
    assert_eq!(resp.status, Status::ValidationError);
    assert_eq!(resp.message, "invalid input");
    assert!(resp.data.is_none());
  }

  #[test]
  fn test_response_map_data() {
    let resp = Response::success("hello", Some("ok"));
    let mapped = resp.map_data(|s| s.len());
    assert_eq!(mapped.status, Status::Success);
    assert_eq!(mapped.message, "ok");
    assert_eq!(mapped.data, Some(5)); // "hello".len() == 5
  }

  #[test]
  fn test_response_ok_with_data() {
    let resp = Response::success("result_data", Some("success message"));
    assert_eq!(resp.status, Status::Success);
    assert_eq!(resp.message, "success message");
    assert_eq!(resp.data, Some("result_data"));
  }

  #[test]
  fn test_response_unauthorized() {
    let resp: Response<()> = Response::unauthorized("token expired");
    assert_eq!(resp.status, Status::Unauthorized);
    assert_eq!(resp.message, "token expired");
    assert!(resp.data.is_none());
  }

  #[test]
  fn test_response_forbidden_msg() {
    let resp: Response<()> = Response::forbidden("access denied");
    assert_eq!(resp.status, Status::Forbidden);
    assert_eq!(resp.message, "access denied");
    assert!(resp.data.is_none());
  }
}
