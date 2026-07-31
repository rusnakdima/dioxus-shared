//! Input components: Input, Textarea, Select
//!
//! Styled with Flowbite/TailwindCSS classes

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus-ui")]
use dioxus::events::FormEvent;

// ============================================================================
// Input
// ============================================================================

/// Input component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Input(
    input_type: String,
    placeholder: String,
    label: String,
    value: String,
    disabled: bool,
    error: Option<String>,
    class: String,
    on_input: EventHandler<FormEvent>,
) -> Element {
    let error_class = if error.is_some() {
        "border-red-500 focus:ring-red-500 focus:border-red-500 dark:border-red-500"
    } else {
        ""
    };
    
    let input_id = format!("input-{}", label.replace(" ", "-").to_lowercase());
    
    rsx! {
        div {
            class: "mb-6",
            
            if !label.is_empty() {
                label {
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    r#for: "{input_id}",
                    "{label}"
                }
            }
            
            input {
                id: "{input_id}",
                r#type: "{input_type}",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 {error_class} {class}",
                placeholder: "{placeholder}",
                disabled: disabled,
                value: "{value}",
                oninput: move |evt| { on_input.call(evt); },
            }
            
            if let Some(error_msg) = &error {
                p {
                    class: "mt-1 text-sm text-red-600 dark:text-red-500",
                    "{error_msg}"
                }
            }
        }
    }
}

// ============================================================================
// Textarea
// ============================================================================

/// Textarea component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Textarea(
    placeholder: String,
    label: String,
    value: String,
    rows: usize,
    disabled: bool,
    error: Option<String>,
    class: String,
    on_input: EventHandler<FormEvent>,
) -> Element {
    let error_class = if error.is_some() {
        "border-red-500 focus:ring-red-500 focus:border-red-500"
    } else {
        ""
    };
    
    let textarea_id = format!("textarea-{}", label.replace(" ", "-").to_lowercase());
    
    rsx! {
        div {
            class: "mb-6",
            
            if !label.is_empty() {
                label {
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    r#for: "{textarea_id}",
                    "{label}"
                }
            }
            
            textarea {
                id: "{textarea_id}",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 {error_class} {class}",
                placeholder: "{placeholder}",
                rows: rows,
                disabled: disabled,
                oninput: move |evt| { on_input.call(evt); },
                "{value}"
            }
            
            if let Some(error_msg) = &error {
                p {
                    class: "mt-1 text-sm text-red-600 dark:text-red-500",
                    "{error_msg}"
                }
            }
        }
    }
}

// ============================================================================
// Select
// ============================================================================

/// Option for Select component
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Select component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Select(
    options: Vec<SelectOption>,
    value: String,
    label: String,
    placeholder: String,
    disabled: bool,
    class: String,
    on_change: EventHandler<String>,
) -> Element {
    let select_id = format!("select-{}", label.replace(" ", "-").to_lowercase());
    
    rsx! {
        div {
            class: "mb-6",
            
            if !label.is_empty() {
                label {
                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                    r#for: "{select_id}",
                    "{label}"
                }
            }
            
            select {
                id: "{select_id}",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 {class}",
                disabled: disabled,
                onchange: move |evt| { on_change.call(evt.value().to_string()); },
                
                // Placeholder option
                option {
                    value: "",
                    disabled: true,
                    selected: value.is_empty(),
                    "{placeholder}"
                }
                
                // Options
                for opt in options {
                    option {
                        value: "{opt.value}",
                        disabled: opt.disabled,
                        selected: opt.value == value,
                        "{opt.label}"
                    }
                }
            }
        }
    }
}
