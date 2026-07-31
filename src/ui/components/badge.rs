//! Badge component
//!
//! Small status indicators with different color variants

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::badge as badge_classes;

/// Badge style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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
            BadgeVariant::Info => "bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300",
        }
    }
}

/// Badge component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Badge(label: String, variant: BadgeVariant, class: String) -> Element {
    let base_classes = variant.as_classes();
    let classes = format!("{} {}", base_classes, class).trim().to_string();
    
    rsx! {
        span {
            class: "{classes}",
            "{label}"
        }
    }
}
