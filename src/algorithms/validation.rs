/// Pure validation and sanitization helpers exposed for free-form use.
///
/// Note: `ValidationAlgorithm` is intentionally a zero-sized type so its
/// methods are namespaced like `ValidationAlgorithm::validate_email(...)`.
pub struct ValidationAlgorithm;

use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]+$").unwrap());

impl ValidationAlgorithm {
    pub fn validate_input(input: &str, max_length: usize) -> bool {
        input.len() <= max_length && !input.is_empty()
    }

    pub fn validate_email(email: &str) -> bool {
        EMAIL_RE.is_match(email)
    }

    pub fn sanitize_input(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
            .collect()
    }

    pub fn calculate_percentage(completed: i32, total: i32) -> i32 {
        if total == 0 {
            return 0;
        }
        ((completed as f32 / total as f32) * 100.0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid_addresses() {
        assert!(ValidationAlgorithm::validate_email("test@example.com"));
        assert!(ValidationAlgorithm::validate_email(
            "user.name@domain.co.uk"
        ));
        assert!(ValidationAlgorithm::validate_email("a@b.c"));
        assert!(ValidationAlgorithm::validate_email("user+tag@domain.org"));
        assert!(ValidationAlgorithm::validate_email("name123@sub.domain.io"));
    }

    #[test]
    fn test_validate_email_invalid_addresses() {
        assert!(!ValidationAlgorithm::validate_email("invalid"));
        assert!(!ValidationAlgorithm::validate_email("no@domain"));
        assert!(!ValidationAlgorithm::validate_email(""));
        assert!(!ValidationAlgorithm::validate_email("plainaddress"));
        assert!(!ValidationAlgorithm::validate_email("@example.com"));
        assert!(!ValidationAlgorithm::validate_email("test@"));
        assert!(!ValidationAlgorithm::validate_email("test@.com"));
    }

    #[test]
    fn test_sanitize_input_preserves_valid_chars() {
        assert_eq!(ValidationAlgorithm::sanitize_input("Hello123"), "Hello123");
        assert_eq!(
            ValidationAlgorithm::sanitize_input("hello world"),
            "hello world"
        );
        assert_eq!(
            ValidationAlgorithm::sanitize_input("file-name.txt"),
            "file-nametxt"
        );
    }

    #[test]
    fn test_sanitize_input_removes_sql_injection() {
        assert_eq!(
            ValidationAlgorithm::sanitize_input("'; DROP TABLE users; --"),
            " DROP TABLE users --"
        );
        assert_eq!(ValidationAlgorithm::sanitize_input("1 OR 1=1"), "1 OR 11");
        assert_eq!(
            ValidationAlgorithm::sanitize_input("' UNION SELECT *"),
            " UNION SELECT "
        );
    }

    #[test]
    fn test_sanitize_input_removes_xss_attempts() {
        assert_eq!(
            ValidationAlgorithm::sanitize_input("<script>alert('xss')</script>"),
            "scriptalertxssscript"
        );
        assert_eq!(
            ValidationAlgorithm::sanitize_input("javascript:void(0)"),
            "javascriptvoid0"
        );
        assert_eq!(
            ValidationAlgorithm::sanitize_input("<img src=x onerror=alert(1)>"),
            "img srcx onerroralert1"
        );
    }

    #[test]
    fn test_sanitize_input_empty() {
        assert_eq!(ValidationAlgorithm::sanitize_input(""), "");
        assert_eq!(ValidationAlgorithm::sanitize_input("!@#$%^&*()"), "");
    }

    #[test]
    fn test_calculate_percentage_normal() {
        assert_eq!(ValidationAlgorithm::calculate_percentage(50, 100), 50);
        assert_eq!(ValidationAlgorithm::calculate_percentage(1, 3), 33);
        assert_eq!(ValidationAlgorithm::calculate_percentage(3, 3), 100);
    }

    #[test]
    fn test_calculate_percentage_zero_total() {
        assert_eq!(ValidationAlgorithm::calculate_percentage(10, 0), 0);
    }

    #[test]
    fn test_validate_input_valid() {
        assert!(ValidationAlgorithm::validate_input("hello", 10));
        assert!(ValidationAlgorithm::validate_input("hello", 5));
    }

    #[test]
    fn test_validate_input_exceeds_max_length() {
        assert!(!ValidationAlgorithm::validate_input("hello world", 5));
    }

    #[test]
    fn test_validate_input_empty() {
        assert!(!ValidationAlgorithm::validate_input("", 10));
    }
}
