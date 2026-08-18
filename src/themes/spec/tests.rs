//! Spec snapshot tests for theme variants
//!
//! These tests verify that the AbstractTokens values for each variant
//! match the expected snapshot files in the snapshots/ directory.

#[allow(unused_imports)]
use crate::themes::{AbstractTokens, ThemeVariant};

/// Load a snapshot file and deserialize it into AbstractTokens
#[allow(dead_code)]
fn load_snapshot(variant: &str, mode: &str) -> AbstractTokens {
    let path = format!("src/themes/spec/snapshots/{}-{}.json", variant, mode);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read snapshot: {}", path));
    serde_json::from_str(&content).unwrap_or_else(|_| panic!("Failed to parse snapshot: {}", path))
}

/// Test Nord light tokens match snapshot
#[test]
fn test_nord_light_tokens() {
    let expected = load_snapshot("nord", "light");
    let actual = ThemeVariant::Nord.light_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test Nord dark tokens match snapshot
#[test]
fn test_nord_dark_tokens() {
    let expected = load_snapshot("nord", "dark");
    let actual = ThemeVariant::Nord.dark_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test TokyoNight light tokens match snapshot
#[test]
fn test_tokyo_night_light_tokens() {
    let expected = load_snapshot("tokyo-night", "light");
    let actual = ThemeVariant::TokyoNight.light_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test TokyoNight dark tokens match snapshot
#[test]
fn test_tokyo_night_dark_tokens() {
    let expected = load_snapshot("tokyo-night", "dark");
    let actual = ThemeVariant::TokyoNight.dark_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test Catppuccin light tokens match snapshot
#[test]
fn test_catppuccin_light_tokens() {
    let expected = load_snapshot("catppuccin", "light");
    let actual = ThemeVariant::Catppuccin.light_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test Catppuccin dark tokens match snapshot
#[test]
fn test_catppuccin_dark_tokens() {
    let expected = load_snapshot("catppuccin", "dark");
    let actual = ThemeVariant::Catppuccin.dark_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test RosePine light tokens match snapshot
#[test]
fn test_rose_pine_light_tokens() {
    let expected = load_snapshot("rose-pine", "light");
    let actual = ThemeVariant::RosePine.light_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test RosePine dark tokens match snapshot
#[test]
fn test_rose_pine_dark_tokens() {
    let expected = load_snapshot("rose-pine", "dark");
    let actual = ThemeVariant::RosePine.dark_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test Linear light tokens match snapshot
#[test]
fn test_linear_light_tokens() {
    let expected = load_snapshot("linear", "light");
    let actual = ThemeVariant::Linear.light_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Test Linear dark tokens match snapshot
#[test]
fn test_linear_dark_tokens() {
    let expected = load_snapshot("linear", "dark");
    let actual = ThemeVariant::Linear.dark_tokens();
    assert_eq!(actual.bg_primary, expected.bg_primary);
    assert_eq!(actual.text_primary, expected.text_primary);
    assert_eq!(actual.accent_primary, expected.accent_primary);
    assert_eq!(actual.success, expected.success);
    assert_eq!(actual.radius, expected.radius);
}

/// Verify that Brutalism dark status colors are all distinct.
/// This was a bug in T3 where success/warning/error/info were all #000000.
#[test]
fn test_brutalism_dark_status_distinguishable() {
    let tokens = ThemeVariant::Brutalism.dark_tokens();
    // All 4 should be distinct
    assert_ne!(
        tokens.success, tokens.warning,
        "success and warning must differ"
    );
    assert_ne!(
        tokens.success, tokens.error,
        "success and error must differ"
    );
    assert_ne!(tokens.success, tokens.info, "success and info must differ");
    assert_ne!(
        tokens.warning, tokens.error,
        "warning and error must differ"
    );
    assert_ne!(tokens.warning, tokens.info, "warning and info must differ");
    assert_ne!(tokens.error, tokens.info, "error and info must differ");
}
