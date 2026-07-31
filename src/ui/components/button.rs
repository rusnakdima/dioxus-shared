//! Button component with multiple variants
//!
//! Supports: solid, outlined, text, tonal variants
//! Styled with Flowbite/TailwindCSS classes

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus-ui")]
use dioxus::events::MouseEvent;
#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::button as btn;

/// Button style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ButtonVariant {
    #[default]
    Solid,
    Outlined,
    Text,
    Tonal,
    Danger,
}


impl ButtonVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            ButtonVariant::Solid => btn::PRIMARY,
            ButtonVariant::Outlined => btn::OUTLINED,
            ButtonVariant::Text => btn::TEXT,
            ButtonVariant::Tonal => btn::TONAL,
            ButtonVariant::Danger => btn::DANGER,
        }
    }
}

/// Button component - use as a regular function in rsx!
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Button(
    label: String,
    variant: ButtonVariant,
    disabled: bool,
    loading: bool,
    class: String,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let base_classes = variant.as_classes();
    let disabled_classes = if disabled { "opacity-50 cursor-not-allowed" } else { "" };
    
    let classes = format!("{} {} {}", base_classes, disabled_classes, class)
        .trim()
        .to_string();
    
    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            disabled: disabled,
            onclick: move |evt| { on_click.call(evt); },
            
            if loading {
                span {
                    class: "me-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent"
                }
            }
            
            "{label}"
        }
    }
}



/// Struct-based button for non-Dioxus usage
#[derive(Debug, Clone)]
pub struct ButtonConfig {
    pub label: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub loading: bool,
    pub class: String,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            label: "Button".to_string(),
            variant: ButtonVariant::Solid,
            disabled: false,
            loading: false,
            class: String::new(),
        }
    }
}

impl ButtonConfig {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}
