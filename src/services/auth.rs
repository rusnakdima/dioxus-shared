//! Authentication service
//!
//! Provides session and user state via `Arc<RwLock<...>>` — works in both UI and
//! daemon/CLI contexts without requiring Dioxus Signals.
//!
//! # Initialization Pattern
//!
//! ```rust,ignore
//! use dioxus_shared::services::AuthService;
//!
//! let auth = AuthService::new();
//! ```

use std::sync::Arc;
use std::sync::OnceLock;

use crate::entities::session::{session_is_expired, Session};
use crate::entities::User;
use tokio::sync::RwLock;

/// Global auth service instance.
static AUTH_SERVICE: OnceLock<AuthService> = OnceLock::new();

/// Returns the global `AuthService` instance.
pub fn global() -> &'static AuthService {
    AUTH_SERVICE.get_or_init(AuthService::new)
}

/// Thread-safe authentication service. Works in any context (UI, daemon, CLI).
#[derive(Clone)]
pub struct AuthService {
    session: Arc<RwLock<Option<Session>>>,
    current_user: Arc<RwLock<Option<User>>>,
}

impl AuthService {
    /// Create a new `AuthService` with `None` session and user.
    pub fn new() -> Self {
        Self {
            session: Arc::new(RwLock::new(None)),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    /// Returns the current session.
    pub async fn session(&self) -> Option<Session> {
        self.session.read().await.clone()
    }

    /// Returns the current user.
    pub async fn current_user(&self) -> Option<User> {
        self.current_user.read().await.clone()
    }

    /// Set the session.
    pub async fn set_session(&self, session: Option<Session>) {
        *self.session.write().await = session;
    }

    /// Set the current user.
    pub async fn set_current_user(&self, user: Option<User>) {
        *self.current_user.write().await = user;
    }

    /// Placeholder login — calls backend in a full implementation.
    pub async fn login(&self, _email: &str, _password: &str) -> Result<User, String> {
        Err("Not implemented".into())
    }

    /// Clear session and current user.
    pub async fn logout(&self) {
        *self.session.write().await = None;
        *self.current_user.write().await = None;
    }

    /// Returns `true` if a non-expired session is present.
    pub async fn is_authenticated(&self) -> bool {
        if let Some(session) = self.session.read().await.as_ref() {
            !session_is_expired(session)
        } else {
            false
        }
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::session::session_is_expired;
    use chrono::{Duration, Utc};

    fn make_test_session(expires_in_secs: i64) -> Session {
        Session {
            id: "session-1".into(),
            user_id: "user-1".into(),
            token: "test-token-abc123".into(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(expires_in_secs),
        }
    }

    fn make_test_user() -> User {
        User {
            id: "user-1".into(),
            email: "testuser@example.com".into(),
            name: "Test User".into(),
            role: crate::entities::UserRole::Editor,
        }
    }

    #[tokio::test]
    async fn test_session_set_and_get() {
        let auth = AuthService::new();
        let session = make_test_session(3600);

        auth.set_session(Some(session.clone())).await;
        let retrieved = auth.session().await;

        assert!(retrieved.is_some());
        let s = retrieved.unwrap();
        assert_eq!(s.id, "session-1");
        assert_eq!(s.token, "test-token-abc123");
    }

    #[tokio::test]
    async fn test_current_user_set_and_get() {
        let auth = AuthService::new();
        let user = make_test_user();

        auth.set_current_user(Some(user.clone())).await;
        let retrieved = auth.current_user().await;

        assert!(retrieved.is_some());
        let u = retrieved.unwrap();
        assert_eq!(u.id, "user-1");
        assert_eq!(u.email, "testuser@example.com");
        assert_eq!(u.role, crate::entities::UserRole::Editor);
    }

    #[tokio::test]
    async fn test_is_authenticated_false_when_no_session() {
        let auth = AuthService::new();
        assert!(!auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_is_authenticated_true_with_valid_session() {
        let auth = AuthService::new();
        auth.set_session(Some(make_test_session(3600))).await;
        assert!(auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_is_authenticated_false_with_expired_session() {
        let auth = AuthService::new();
        // Expired 1 hour ago
        auth.set_session(Some(make_test_session(-3600))).await;
        assert!(!auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_session_expiration_direct_expired() {
        let expired = make_test_session(-3600);
        assert!(session_is_expired(&expired));
    }

    #[tokio::test]
    async fn test_session_expiration_direct_not_expired() {
        let not_expired = make_test_session(3600);
        assert!(!session_is_expired(&not_expired));
    }

    #[tokio::test]
    async fn test_session_expiration_far_future() {
        let far_future = make_test_session(86400 * 365);
        assert!(!session_is_expired(&far_future));
    }

    #[tokio::test]
    async fn test_session_expiration_just_expired() {
        let just_expired = make_test_session(-1);
        assert!(session_is_expired(&just_expired));
    }

    #[tokio::test]
    async fn test_logout_clears_session_and_user() {
        let auth = AuthService::new();
        auth.set_session(Some(make_test_session(3600))).await;
        auth.set_current_user(Some(make_test_user())).await;

        auth.logout().await;

        assert!(auth.session().await.is_none());
        assert!(auth.current_user().await.is_none());
        assert!(!auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_set_session_none_clears_session() {
        let auth = AuthService::new();
        auth.set_session(Some(make_test_session(3600))).await;
        auth.set_session(None).await;
        assert!(auth.session().await.is_none());
        assert!(!auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_set_current_user_none_clears_user() {
        let auth = AuthService::new();
        auth.set_current_user(Some(make_test_user())).await;
        auth.set_current_user(None).await;
        assert!(auth.current_user().await.is_none());
    }

    #[tokio::test]
    async fn test_auth_service_clone_shares_state() {
        let auth1 = AuthService::new();
        let auth2 = auth1.clone();

        auth1.set_session(Some(make_test_session(3600))).await;
        auth1.set_current_user(Some(make_test_user())).await;

        // Clones share the same underlying Arc state
        assert!(auth2.session().await.is_some());
        assert!(auth2.current_user().await.is_some());
        assert!(auth2.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_separate_auth_services_have_independent_state() {
        let auth1 = AuthService::new();
        let auth2 = AuthService::new();

        auth1.set_session(Some(make_test_session(3600))).await;

        // auth1 has session
        assert!(auth1.is_authenticated().await);
        // auth2 is independent
        assert!(!auth2.is_authenticated().await);
        assert!(auth2.session().await.is_none());
    }

    #[tokio::test]
    async fn test_login_returns_not_implemented() {
        let auth = AuthService::new();
        let result = auth.login("test@example.com", "password123").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not implemented");
    }

    #[tokio::test]
    async fn test_authenticated_session_carries_user_id() {
        let auth = AuthService::new();
        let session = make_test_session(3600);
        let user_id = session.user_id.clone();

        auth.set_session(Some(session)).await;

        let retrieved = auth.session().await.unwrap();
        assert_eq!(retrieved.user_id, user_id);
        assert!(auth.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_session_token_preserved_across_set_get() {
        let auth = AuthService::new();
        let token = "my-secret-token-xyz789";
        let session = Session {
            id: "sess-99".into(),
            user_id: "u-99".into(),
            token: token.into(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
        };

        auth.set_session(Some(session.clone())).await;
        let retrieved = auth.session().await.unwrap();

        assert_eq!(retrieved.token, token);
        assert_eq!(retrieved.id, "sess-99");
        assert_eq!(retrieved.user_id, "u-99");
    }
}
