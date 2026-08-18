//! Semantic variant system for SDUI schema.
//!
//! UniversalVariant: applies to ALL semantic types
//! ComponentVariant: per-type variant sets (button, badge, card, input)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal variants — available for every component type.
/// E.g. a button, card, badge, or input can all use "primary", "danger", etc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum UniversalVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
    Warning,
}

impl UniversalVariant {
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "primary" => Some(UniversalVariant::Primary),
            "secondary" => Some(UniversalVariant::Secondary),
            "outline" => Some(UniversalVariant::Outline),
            "ghost" => Some(UniversalVariant::Ghost),
            "danger" => Some(UniversalVariant::Danger),
            "success" => Some(UniversalVariant::Success),
            "warning" => Some(UniversalVariant::Warning),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UniversalVariant::Primary => "primary",
            UniversalVariant::Secondary => "secondary",
            UniversalVariant::Outline => "outline",
            UniversalVariant::Ghost => "ghost",
            UniversalVariant::Danger => "danger",
            UniversalVariant::Success => "success",
            UniversalVariant::Warning => "warning",
        }
    }
}

/// Component-specific variants — only valid for specific component types.
/// Maps component name → list of valid variant names.
pub type ComponentVariantMap = HashMap<&'static str, Vec<&'static str>>;

/// Returns the complete component variant registry.
pub fn component_variants() -> ComponentVariantMap {
    let mut m = HashMap::new();

    // Button variants
    m.insert(
        "button",
        vec![
            "primary",
            "secondary",
            "outline",
            "ghost",
            "danger",
            "success",
            "warning",
            "gradient",
            "floating",
            "circle",
            "icon",
        ],
    );
    m.insert(
        "action-button",
        vec![
            "primary",
            "secondary",
            "outline",
            "ghost",
            "danger",
            "success",
            "warning",
            "gradient",
            "floating",
            "circle",
            "icon",
        ],
    );

    // Badge variants
    m.insert(
        "badge",
        vec![
            "primary",
            "secondary",
            "success",
            "warning",
            "danger",
            "info",
            "outline",
            "gradient",
        ],
    );
    m.insert(
        "chip",
        vec![
            "primary",
            "secondary",
            "success",
            "warning",
            "danger",
            "info",
            "outline",
        ],
    );

    // Card variants
    m.insert("card", vec!["elevated", "flat", "outline", "colored"]);

    // Input variants
    m.insert("input", vec!["default", "filled", "outline"]);
    m.insert("textarea", vec!["default", "filled", "outline"]);
    m.insert("select", vec!["default", "filled", "outline"]);
    m.insert("action-input", vec!["default", "filled", "outline"]);
    m.insert("action-textarea", vec!["default", "filled", "outline"]);
    m.insert("action-select", vec!["default", "filled", "outline"]);

    // Icon variants
    m.insert("icon", vec!["solid", "outline", "filled"]);

    // Avatar variants
    m.insert("avatar", vec!["circle", "square", "rounded"]);

    // Toggle variants
    m.insert("toggle", vec!["default", "checked", "disabled"]);

    // Slider/Range variants
    m.insert("slider", vec!["default", "disabled"]);
    m.insert("range", vec!["default", "disabled"]);

    // Progress variants
    m.insert(
        "progress",
        vec!["default", "success", "warning", "danger", "info"],
    );
    m.insert(
        "progress-bar",
        vec!["default", "success", "warning", "danger", "info"],
    );

    // Table variants
    m.insert("table", vec!["default", "striped", "bordered"]);

    // Modal variants
    m.insert("modal", vec!["default", "alert", "confirm"]);

    // Dropdown variants
    m.insert("dropdown", vec!["default", "mega"]);

    // Pagination variants
    m.insert("pagination", vec!["default"]);

    // Breadcrumb variants
    m.insert("breadcrumb", vec!["default", "slash", "chevron"]);

    // Spinner/Loading variants
    m.insert("spinner", vec!["default", "circle", "dots"]);

    // Tooltip variants
    m.insert("tooltip", vec!["default", "dark", "light"]);

    // Tabs variants
    m.insert("tabs", vec!["default", "pills", "underline", "boxed"]);

    // Alert variants
    m.insert(
        "alert",
        vec!["default", "success", "warning", "danger", "info"],
    );

    // Empty state
    m.insert("empty-state", vec!["default", "illustrated"]);

    // Stepper variants
    m.insert("stepper", vec!["default", "numeric", "icon"]);

    m
}

/// Check if a variant name is valid for a given component type.
/// If the component has no specific variants defined, checks UniversalVariant.
pub fn is_valid_variant(component: &str, variant: &str) -> bool {
    // First check component-specific variants
    if let Some(variants) = component_variants().get(component) {
        return variants.contains(&variant);
    }
    // Fall back to universal variants
    UniversalVariant::from_str(variant).is_some()
}
