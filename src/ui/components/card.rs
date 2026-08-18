//! Card component
//!
//! Provides card containers with different styles

#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::card as card_classes;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Card style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Elevated,
    Bordered,
}

#[cfg(feature = "dioxus-ui")]
impl CardVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            CardVariant::Default => card_classes::DEFAULT,
            CardVariant::Elevated => card_classes::ELEVATED,
            CardVariant::Bordered => card_classes::BORDERED,
        }
    }
}

/// Card component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Card(
    title: String,
    subtitle: String,
    variant: CardVariant,
    class: String,
    children: Element,
) -> Element {
    let base_classes = variant.as_classes();
    let classes = format!("{} {}", base_classes, class).trim().to_string();

    rsx! {
        div {
            class: "{classes}",
            style: "background-color: var(--color-bg-surface);",

            if !title.is_empty() || !subtitle.is_empty() {
                div {
                    class: "px-4 py-3 sm:px-6 sm:py-4",

                    if !title.is_empty() {
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-white",
                            "{title}"
                        }
                    }

                    if !subtitle.is_empty() {
                        p {
                            class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            "{subtitle}"
                        }
                    }
                }
            }

            div {
                class: "px-4 py-3 sm:px-6 sm:py-4",
                {children}
            }
        }
    }
}

/// Struct-based card for non-Dioxus usage
#[derive(Debug, Clone)]
pub struct CardConfig {
    pub title: String,
    pub subtitle: String,
    pub variant: CardVariant,
    pub class: String,
}

impl Default for CardConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            subtitle: String::new(),
            variant: CardVariant::Default,
            class: String::new(),
        }
    }
}

impl CardConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}
