//! Badge component
//!
//! Small status indicators with different color variants

#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::badge as badge_classes;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Badge style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

#[cfg(feature = "dioxus-ui")]
impl BadgeVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => badge_classes::DEFAULT,
            BadgeVariant::Success => badge_classes::SUCCESS,
            BadgeVariant::Warning => badge_classes::WARNING,
            BadgeVariant::Error => badge_classes::ERROR,
            BadgeVariant::Info => badge_classes::INFO,
        }
    }

    pub fn as_style(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "background-color: var(--color-accent-subtle); color: var(--color-accent-primary);"
            }
            BadgeVariant::Success => {
                "background-color: var(--color-success-subtle); color: var(--color-success);"
            }
            BadgeVariant::Warning => {
                "background-color: var(--color-warning-subtle); color: var(--color-warning);"
            }
            BadgeVariant::Error => {
                "background-color: var(--color-error-subtle); color: var(--color-error);"
            }
            BadgeVariant::Info => {
                "background-color: var(--color-accent-subtle); color: var(--color-accent-primary);"
            }
        }
    }
}

/// Badge component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Badge(label: String, variant: BadgeVariant, class: String) -> Element {
    let base_classes = variant.as_classes();
    let variant_style = variant.as_style();
    let classes = format!("{} {}", base_classes, class).trim().to_string();

    rsx! {
        span {
            class: "{classes}",
            style: "{variant_style}",
            "{label}"
        }
    }
}
