//! Environment configuration service
//!
//! Reads configuration from environment variables or `.env` files.
//! Use [`EnvConfig::from_env()`] to load at startup, then convert to
//! [`AppConfig`](crate::entities::AppConfig) via [`EnvConfig::to_app_config()`].

use crate::entities::AppConfig;
use crate::themes::ThemeVariant;

/// Application environment configuration.
/// Populated from `APP_NAME`, `API_BASE_URL`, and `LOG_LEVEL` env vars.
pub struct EnvConfig {
    pub app_name: String,
    pub api_base_url: String,
    pub log_level: String,
}

impl EnvConfig {
    /// Read configuration from environment variables.
    ///
    /// Defaults if unset:
    /// - `APP_NAME` → `"app"`
    /// - `API_BASE_URL` → `"http://localhost:8080"`
    /// - `LOG_LEVEL` → `"info"`
    ///
    /// ```rust
    /// use dioxus_shared::env::EnvConfig;
    ///
    /// let config = EnvConfig::from_env();
    /// println!("app: {}", config.app_name);
    /// ```
    pub fn from_env() -> Self {
        Self {
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "app".into()),
            api_base_url: std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".into()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        }
    }

    /// Convert to an [`AppConfig`] entity with a provided default theme.
    pub fn to_app_config(&self, default_theme: ThemeVariant) -> AppConfig {
        AppConfig {
            app_name: self.app_name.clone(),
            api_base_url: self.api_base_url.clone(),
            theme_default: default_theme,
            log_level: self.log_level.clone(),
        }
    }
}
