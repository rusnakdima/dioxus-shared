/// Pure validation and sanitization helpers exposed for free-form use.
///
/// Note: `ValidationAlgorithm` is intentionally a zero-sized type so its
/// methods are namespaced like `ValidationAlgorithm::validate_email(...)`.
pub struct ValidationAlgorithm;

use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]+$").unwrap()
});

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