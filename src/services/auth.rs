//! Authentication service
//!
//! Provides session and user state via Dioxus signals when the `dioxus-ui` feature
//! is enabled. In non-UI contexts, `AuthService::new()` returns a no-op unit struct.
//!
//! # Initialization Pattern
//!
//! ```rust
//! use dioxus_shared::services::AuthService;
//!
//! #[cfg(feature = "dioxus-ui")]
//! let auth = AuthService::new();
//!
//! #[cfg(not(feature = "dioxus-ui"))]
//! let auth = AuthService::new();
//! ```
//!
//! Both branches call `AuthService::new()` — the concrete return type differs
//! based on the feature flag. Use `#[cfg(feature = "dioxus-ui")]` guards when
//! accessing signal-based methods (`session()`, `current_user()`, `is_authenticated()`).

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::{Signal, ReadableExt, WritableExt};

#[cfg(feature = "dioxus-ui")]
use crate::entities::session::{session_is_expired, Session};
#[cfg(feature = "dioxus-ui")]
use crate::entities::User;

/// Signal-backed authentication service. Available only with `dioxus-ui` feature.
#[cfg(feature = "dioxus-ui")]
#[derive(Clone)]
pub struct AuthService {
    session: Signal<Option<Session>>,
    current_user: Signal<Option<User>>,
}

#[cfg(feature = "dioxus-ui")]
impl AuthService {
    /// Create a new `AuthService` with `None` session and user.
    ///
    /// ```rust
    /// use dioxus_shared::services::AuthService;
    /// let auth = AuthService::new();
    /// ```
    pub fn new() -> Self {
        Self {
            session: Signal::new(None),
            current_user: Signal::new(None),
        }
    }

    /// Returns the current session signal.
    pub fn session(&self) -> &Signal<Option<Session>> {
        &self.session
    }

    /// Returns the current user signal.
    pub fn current_user(&self) -> &Signal<Option<User>> {
        &self.current_user
    }

    /// Placeholder login — calls backend in a full implementation.
    pub fn login(&self, _email: &str, _password: &str) -> Result<User, String> {
        Err("Not implemented".into())
    }

    /// Clear session and current user.
    pub fn logout(&mut self) {
        self.session.set(None);
        self.current_user.set(None);
    }

    /// Returns `true` if a non-expired session is present.
    pub fn is_authenticated(&self) -> bool {
        if let Some(session) = self.session.read().as_ref() {
            !session_is_expired(session)
        } else {
            false
        }
    }
}

/// No-op auth service for non-UI contexts. Available when `dioxus-ui` is not enabled.
#[cfg(not(feature = "dioxus-ui"))]
pub struct AuthService;

#[cfg(not(feature = "dioxus-ui"))]
impl AuthService {
    /// Returns a no-op `AuthService` that performs no state management.
    pub fn new() -> Self {
        Self
    }
}
