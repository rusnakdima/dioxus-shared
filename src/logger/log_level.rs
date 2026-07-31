//! Log level enumeration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum LogLevel {
  Debug,
  #[default]
  Info,
  Warn,
  Error,
}

impl LogLevel {
  pub fn is_enabled(&self, min_level: LogLevel) -> bool {
    let self_val = match self {
      LogLevel::Debug => 0,
      LogLevel::Info => 1,
      LogLevel::Warn => 2,
      LogLevel::Error => 3,
    };
    let min_val = match min_level {
      LogLevel::Debug => 0,
      LogLevel::Info => 1,
      LogLevel::Warn => 2,
      LogLevel::Error => 3,
    };
    self_val >= min_val
  }
}

impl std::fmt::Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LogLevel::Debug => write!(f, "DEBUG"),
      LogLevel::Info => write!(f, "INFO"),
      LogLevel::Warn => write!(f, "WARN"),
      LogLevel::Error => write!(f, "ERROR"),
    }
  }
}

