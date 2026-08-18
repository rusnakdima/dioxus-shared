//! Environment variable loading with compile-time defaults and .env support.

use crate::entities::AppConfig;
use crate::themes::ThemeVariant;
use once_cell::sync::Lazy;

/// Common environment configuration shared across all Dioxus projects.
#[derive(Debug, Clone)]
pub struct EnvConfig {
    /// Application ID
    pub app_id: String,
    /// Application name
    pub app_name: String,
    /// Data directory path
    pub data_dir: String,
    /// Log level (default: info)
    pub log_level: String,
    /// Whether logging is enabled (default: true)
    pub log_enabled: bool,
    /// Environment (default: development)
    pub environment: String,
    /// API base URL (used by services that need to call back to the app)
    pub api_base_url: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            app_id: std::env::var("APP_ID").unwrap_or_else(|_| "dioxus-app".to_string()),
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "Dioxus App".to_string()),
            data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            log_enabled: std::env::var("LOG_ENABLED")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
        }
    }
}

impl EnvConfig {
    /// Load environment from std::env vars with compile-time defaults.
    /// Also attempts to load from .env file if dotenv is available.
    pub fn load() -> Self {
        #[cfg(feature = "dotenvy")]
        let _ = dotenvy::dotenv();
        Self::default()
    }

    /// Get a typed environment variable with a default fallback.
    pub fn or(&self, key: &str, default: &'static str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Get a typed environment variable.
    pub fn get<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        std::env::var(key).ok()?.parse().ok()
    }

    /// Check if running in a specific environment.
    pub fn is_environment(&self, env: &str) -> bool {
        self.environment == env
    }

    /// Check if running in development mode.
    pub fn is_development(&self) -> bool {
        self.is_environment("development")
    }

    /// Check if running in production mode.
    pub fn is_production(&self) -> bool {
        self.is_environment("production")
    }

    /// Read configuration from environment variables only (no .env file loading).
    ///
    /// Defaults if unset:
    /// - `APP_NAME` → `"app"`
    /// - `API_BASE_URL` → `"http://localhost:8080"`
    /// - `LOG_LEVEL` → `"info"`
    pub fn from_env() -> Self {
        Self {
            app_id: std::env::var("APP_ID").unwrap_or_else(|_| "dioxus-app".into()),
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "app".into()),
            data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".into()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
            log_enabled: std::env::var("LOG_ENABLED")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into()),
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
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

/// Global environment config — initialized once at startup.
pub static ENV: Lazy<EnvConfig> = Lazy::new(EnvConfig::load);

/// Macro to load environment config at compile time for use in const contexts.
#[macro_export]
macro_rules! env_var {
    ($key:expr) => {
        std::env!($key)
    };
    ($key:expr, $default:expr) => {
        std::env::var($key).unwrap_or_else(|_| $default.to_string())
    };
}

#[cfg(feature = "dotenvy")]
pub fn init_env() {
    let _ = dotenvy::dotenv();
}

#[cfg(not(feature = "dotenvy"))]
pub fn init_env() {}
