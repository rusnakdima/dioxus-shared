pub mod graph;
pub mod registry;
pub mod sanitization;
pub mod search;
pub mod sorting;
pub mod validation;

pub use registry::{Algorithm, AlgorithmRegistry, AlgorithmInput, AlgorithmOutput};

pub use graph::Graph;
pub use sanitization::{sanitize_for_mongo, sanitize_for_mongo_owned, sanitize_for_overlay};
pub use search::SearchAlgorithm;
pub use sorting::{
  bubble_sort, bubble_sort_by, insertion_sort, insertion_sort_by, merge_sort, merge_sort_by,
  quick_sort, quick_sort_by,
};
pub use validation::ValidationAlgorithm;

/// High-level algorithm execution helper.
///
/// Calls `registry.execute(name, input)` and returns the output JSON or an error
/// string. This is the primary entry point for application code that needs to
/// invoke built-in algorithms (sort, search, graph, validate, sanitize).
///
/// The `data` argument is passed as the algorithm's input array.
/// The `field` and `order` arguments correspond to `AlgorithmInput::field/order`.
///
/// # When to use `algo_execute()` vs direct imports
///
/// - **`algo_execute()`**: Use when the algorithm name is determined at runtime
///   (e.g., from config, user preference, or a pipeline). Enables dynamic routing
///   without compile-time knowledge of which algorithm is selected.
/// - **Direct imports** (`use dioxus_shared::algorithms::{bubble_sort, ...}`): Use when
///   the algorithm is known at compile time. Allows dead-code elimination and
///   better inlining. Preferred for hot paths where performance matters.
///
/// ```rust
/// // Runtime algorithm selection (algo_execute)
/// let registry = AlgorithmRegistry::new();
/// let result = algo_execute(&registry, "bubble_sort", data.into(), None, None)?;
///
/// // Compile-time algorithm selection (direct import)
/// use dioxus_shared::algorithms::bubble_sort;
/// let sorted = bubble_sort(&mut data);
/// ```
//
// WHEN TO USE `algo_execute()` vs DIRECT IMPORTS:
//
// Use `algo_execute()` when:
//   - Algorithm is selected dynamically at runtime (e.g., user preference, plugin system)
//   - Building a plugin/extension system where algorithms are discovered at runtime
//   - Writing middleware or generic code that routes to any registered algorithm by name
//   - Cross-platform code where the concrete algorithm may vary per platform
//
// Use direct imports (e.g., `sorting::merge_sort`, `search::binary_search`) when:
//   - Performance-critical hot paths where the algorithm is known at compile time
//   - The algorithm is fixed and will never change at runtime
//   - You want compile-time error checking and IDE autocomplete for the concrete type
//   - The code is inside the algorithms crate itself
//
// Example - runtime selection (use `algo_execute()`):
//   algo_execute(&registry, "merge_sort", data, Some("name"), Some("asc"))?;
//
// Example - compile-time selection (use direct import):
//   let sorted = sorting::merge_sort(data, &sorting::SortBy::Field("name", sorting::SortOrder::Asc));
//
pub fn algo_execute(
  registry: &AlgorithmRegistry,
  name: &str,
  data: serde_json::Value,
  field: Option<&str>,
  order: Option<&str>,
) -> Result<serde_json::Value, String> {
  let input = AlgorithmInput {
    data,
    field: field.map(String::from),
    order: order.map(String::from),
  };
  registry
    .execute(name, input)
    .map(|out| out.data)
    .ok_or_else(|| format!("algorithm not found: {name}"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_input_valid() {
    assert!(ValidationAlgorithm::validate_input("hello", 10));
    assert!(!ValidationAlgorithm::validate_input("", 10)); // empty string is not valid
  }

  #[test]
  fn test_validate_input_exceeds_max_length() {
    assert!(!ValidationAlgorithm::validate_input("hello world", 5));
  }

  #[test]
  fn test_validate_input_empty() {
    assert!(!ValidationAlgorithm::validate_input("", 10));
  }

  #[test]
  fn test_validate_input_exact_length() {
    assert!(ValidationAlgorithm::validate_input("hello", 5));
  }

  #[test]
  fn test_validate_email_valid() {
    assert!(ValidationAlgorithm::validate_email("test@example.com"));
    assert!(ValidationAlgorithm::validate_email("user.name@domain.co.uk"));
    assert!(ValidationAlgorithm::validate_email("a@b.c"));
  }

  #[test]
  fn test_validate_email_invalid() {
    assert!(!ValidationAlgorithm::validate_email("invalid"));
    assert!(!ValidationAlgorithm::validate_email("no@domain"));
    assert!(!ValidationAlgorithm::validate_email("")); // empty string is invalid
  }

  #[test]
  fn test_sanitize_input_keeps_valid_chars() {
    assert_eq!(ValidationAlgorithm::sanitize_input("Hello123"), "Hello123");
    assert_eq!(ValidationAlgorithm::sanitize_input("hello world"), "hello world");
    // Note: '.' is not alphanumeric, space, or hyphen, so it's removed
    assert_eq!(ValidationAlgorithm::sanitize_input("file-name.txt"), "file-nametxt");
  }

  #[test]
  fn test_sanitize_input_removes_invalid_chars() {
    assert_eq!(ValidationAlgorithm::sanitize_input("hello;world"), "helloworld");
    assert_eq!(ValidationAlgorithm::sanitize_input("test@email.com"), "testemailcom");
    assert_eq!(ValidationAlgorithm::sanitize_input("user'name"), "username");
    assert_eq!(ValidationAlgorithm::sanitize_input("path/to/file"), "pathtofile");
  }

  #[test]
  fn test_sanitize_input_preserves_spaces_and_hyphens() {
    assert_eq!(ValidationAlgorithm::sanitize_input("hello - world"), "hello - world");
    assert_eq!(ValidationAlgorithm::sanitize_input("user-name-123"), "user-name-123");
  }

  #[test]
  fn test_sanitize_input_unicode_chars_removed() {
    // Unicode chars that are alphanumeric pass through
    assert_eq!(ValidationAlgorithm::sanitize_input("café"), "café");
    assert_eq!(ValidationAlgorithm::sanitize_input("naïve"), "naïve");
  }

  #[test]
  fn test_sanitize_input_empty() {
    assert_eq!(ValidationAlgorithm::sanitize_input(""), "");
    assert_eq!(ValidationAlgorithm::sanitize_input("!@#$%"), "");
  }
}