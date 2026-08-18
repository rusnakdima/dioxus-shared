//! Progress/ProgressBar component
//!
//! Displays a progress bar with percentage, supporting determinate and indeterminate states.

#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::progress as progress_classes;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Progress size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressSize {
    #[default]
    Md,
    Sm,
    Lg,
}

/// Progress color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

#[cfg(feature = "dioxus-ui")]
impl ProgressSize {
    pub fn as_classes(&self) -> &'static str {
        match self {
            ProgressSize::Sm => progress_classes::SIZE_SM,
            ProgressSize::Md => progress_classes::SIZE_MD,
            ProgressSize::Lg => progress_classes::SIZE_LG,
        }
    }

    pub fn height(&self) -> &'static str {
        match self {
            ProgressSize::Sm => "h-1",
            ProgressSize::Md => "h-2.5",
            ProgressSize::Lg => "h-4",
        }
    }
}

#[cfg(feature = "dioxus-ui")]
impl ProgressVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            ProgressVariant::Default => progress_classes::DEFAULT,
            ProgressVariant::Success => progress_classes::SUCCESS,
            ProgressVariant::Warning => progress_classes::WARNING,
            ProgressVariant::Error => progress_classes::ERROR,
        }
    }

    pub fn fill_style(&self) -> &'static str {
        match self {
            ProgressVariant::Default => "background-color: var(--color-accent-primary);",
            ProgressVariant::Success => "background-color: var(--color-success);",
            ProgressVariant::Warning => "background-color: var(--color-warning);",
            ProgressVariant::Error => "background-color: var(--color-error);",
        }
    }
}

/// Progress component - displays a progress bar with optional percentage label
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Progress(
    /// Current progress value (0-100). None for indeterminate.
    value: Option<i32>,
    /// Size variant
    size: ProgressSize,
    /// Color variant
    variant: ProgressVariant,
    /// Show percentage label
    show_label: bool,
    /// Additional CSS classes
    class: String,
) -> Element {
    let is_determinate = value.is_some();
    let progress_value = value.unwrap_or(0).clamp(0, 100);

    let base_classes = size.as_classes();
    let variant_classes = variant.as_classes();
    let variant_fill_style = variant.fill_style();
    let height_class = size.height();

    let classes = format!("{} {} {}", base_classes, variant_classes, class)
        .trim()
        .to_string();

    if is_determinate {
        rsx! {
            div {
                class: "w-full {classes}",
                role: "progressbar",
                aria_valuenow: "{progress_value}",
                aria_valuemin: "0",
                aria_valuemax: "100",

                div {
                    class: "w-full rounded-full {height_class}",
                    style: "background-color: var(--color-bg-surface);",

                    div {
                        class: "{height_class} rounded-full transition-all duration-300 ease-in-out",
                        style: "width: {progress_value}%; {variant_fill_style}",

                        if show_label {
                            span {
                                class: "flex items-center justify-center h-full text-xs font-medium",
                                style: "color: var(--color-text-inverse);",
                                "{progress_value}%"
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "w-full {classes}",
                role: "progressbar",
                aria_valuenow: "0",
                aria_valuemin: "0",
                aria_valuemax: "100",

                div {
                    class: "w-full rounded-full {height_class}",
                    style: "background-color: var(--color-bg-surface);",

                    div {
                        class: "{height_class} rounded-full animate-pulse",
                        style: "width: 100%; {variant_fill_style}",
                    }
                }
            }
        }
    }
}
