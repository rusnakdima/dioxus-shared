//! Accordion component
//!
//! Provides collapsible accordion sections with accessibility support.
//! Supports default, bordered, and ghost variants.

#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::accordion as accordion_classes;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Accordion style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionVariant {
    #[default]
    Default,
    Bordered,
    Ghost,
}

#[cfg(feature = "dioxus-ui")]
impl AccordionVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            AccordionVariant::Default => accordion_classes::DEFAULT,
            AccordionVariant::Bordered => accordion_classes::BORDERED,
            AccordionVariant::Ghost => accordion_classes::GHOST,
        }
    }
}

/// Accordion item state
#[cfg(feature = "dioxus-ui")]
#[derive(Clone, Copy)]
pub struct AccordionItemState {
    pub expanded: bool,
}

#[cfg(feature = "dioxus-ui")]
impl AccordionItemState {
    pub fn new(expanded: bool) -> Self {
        Self { expanded }
    }
}

/// Single accordion item component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn AccordionItem(
    title: String,
    content: String,
    expanded: bool,
    disabled: bool,
    class: String,
    on_toggle: EventHandler<()>,
) -> Element {
    let content_id = format!(
        "accordion-content-{}",
        title.replace(" ", "-").to_lowercase()
    );
    let button_id = format!(
        "accordion-button-{}",
        title.replace(" ", "-").to_lowercase()
    );

    let border_class = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };

    let chevron_class = if expanded { "rotate-180" } else { "" };

    rsx! {
        div {
            class: "{class}",
            button {
                id: "{button_id}",
                class: "flex items-center justify-between w-full text-left px-4 py-3 focus:outline-none transition-colors {border_class}",
                r#type: "button",
                disabled: disabled,
                onclick: move |_| {
                    if !disabled {
                        on_toggle.call(());
                    }
                },
                aria_expanded: "{expanded}",
                aria_controls: "{content_id}",
                span {
                    class: "text-base font-medium text-gray-900 dark:text-white",
                    "{title}"
                }
                span {
                    class: "material-symbols-rounded text-gray-500 dark:text-gray-400 transition-transform {chevron_class}",
                    "expand_more"
                }
            }
            div {
                id: "{content_id}",
                class: if expanded { "block" } else { "hidden" },
                role: "region",
                aria_labelledby: "{button_id}",
                div {
                    class: "px-4 py-3 text-sm text-gray-600 dark:text-gray-300",
                    "{content}"
                }
            }
        }
    }
}

/// Accordion component - container for multiple accordion items
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Accordion(
    items: Vec<(String, String, bool)>,
    variant: AccordionVariant,
    class: String,
    allow_multiple: bool,
) -> Element {
    let base_classes = variant.as_classes();
    let initial_expanded_states = items
        .iter()
        .map(|(_, _, expanded)| *expanded)
        .collect::<Vec<_>>();
    let mut expanded_states = use_signal(|| initial_expanded_states);

    let classes = format!(
        "{} {} {}",
        base_classes, "divide-y divide-gray-200 dark:divide-gray-700", class
    )
    .trim()
    .to_string();

    rsx! {
        div {
            class: "{classes}",
            role: "tablist",
            aria_label: "Accordion",
            for (i, (title, content, _)) in items.iter().enumerate() {
                AccordionItem {
                    title: title.clone(),
                    content: content.clone(),
                    expanded: *expanded_states.read().get(i).unwrap_or(&false),
                    disabled: false,
                    class: "",
                    on_toggle: move |_| {
                        let mut states = expanded_states.read().clone();
                        if allow_multiple {
                            states[i] = !states[i];
                        } else {
                            // Close all others, only open the clicked one
                            for j in 0..states.len() {
                                states[j] = i == j && !states[i];
                            }
                        }
                        expanded_states.set(states);
                    }
                }
            }
        }
    }
}

/// AccordionItem for schema-driven usage
#[cfg(feature = "dioxus-ui")]
pub fn render_accordion_item(
    element: &crate::schema::CanvasElement,
    classes: &str,
    _is_dark: bool,
) -> Element {
    let title_text = element
        .props
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Accordion Item")
        .to_string();
    let content_text = element
        .props
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let expanded = element
        .props
        .get("expanded")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let disabled = element
        .props
        .get("disabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let content_id = format!(
        "accordion-content-{}",
        title_text.replace(" ", "-").to_lowercase()
    );
    let button_id = format!(
        "accordion-button-{}",
        title_text.replace(" ", "-").to_lowercase()
    );

    let chevron_class = if expanded { "rotate-180" } else { "" };
    let content_display = if expanded { "block" } else { "hidden" };

    rsx! {
        div {
            class: "{classes}",
            button {
                id: "{button_id}",
                class: "flex items-center justify-between w-full text-left px-4 py-3 focus:outline-none transition-colors",
                r#type: "button",
                disabled: disabled,
                onclick: move |_| {
                    // Toggle logic handled by parent Accordion
                },
                aria_expanded: "{expanded}",
                aria_controls: "{content_id}",
                span {
                    class: "text-base font-medium text-gray-900 dark:text-white",
                    "{title_text}"
                }
                span {
                    class: "material-symbols-rounded text-gray-500 dark:text-gray-400 transition-transform {chevron_class}",
                    "expand_more"
                }
            }
            div {
                id: "{content_id}",
                class: "{content_display}",
                role: "region",
                aria_labelledby: "{button_id}",
                div {
                    class: "px-4 py-3 text-sm text-gray-600 dark:text-gray-300",
                    "{content_text}"
                }
            }
        }
    }
}

/// Struct-based accordion item for non-Dioxus usage
#[derive(Debug, Clone)]
pub struct AccordionItemConfig {
    pub title: String,
    pub content: String,
    pub expanded: bool,
    pub disabled: bool,
}

impl Default for AccordionItemConfig {
    fn default() -> Self {
        Self {
            title: "Accordion Item".to_string(),
            content: String::new(),
            expanded: false,
            disabled: false,
        }
    }
}

impl AccordionItemConfig {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Struct-based accordion for non-Dioxus usage
#[derive(Debug, Clone)]
pub struct AccordionConfig {
    pub items: Vec<AccordionItemConfig>,
    pub variant: AccordionVariant,
    pub allow_multiple: bool,
    pub class: String,
}

impl Default for AccordionConfig {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            variant: AccordionVariant::Default,
            allow_multiple: false,
            class: String::new(),
        }
    }
}

impl AccordionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, item: AccordionItemConfig) -> Self {
        self.items.push(item);
        self
    }

    pub fn variant(mut self, variant: AccordionVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn allow_multiple(mut self, allow_multiple: bool) -> Self {
        self.allow_multiple = allow_multiple;
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}
