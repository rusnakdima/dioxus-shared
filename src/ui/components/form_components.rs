//! Form components for SDUI — Input, Textarea, Select and their action-* variants.
//!
//! Extracted from dynamic_renderer.rs to reduce file size and improve maintainability.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;

#[cfg(feature = "dioxus-ui")]
use super::action_bus::ActionBus;

/// Render an Input element.
#[cfg(feature = "dioxus-ui")]
pub fn render_input(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let placeholder_text = element
        .props
        .get("placeholder")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    rsx! {
        input {
            class: "{classes}",
            placeholder: "{placeholder_text}",
            r#type: "text"
        }
    }
}

/// Render a Textarea element.
#[cfg(feature = "dioxus-ui")]
pub fn render_textarea(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let bind_key = element
        .props
        .get("bind")
        .or_else(|| element.props.get("binding"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let placeholder_text = element
        .props
        .get("placeholder")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let is_readonly = element
        .props
        .get("readonly")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let mut bus = use_context::<ActionBus>();
    let initial_value = if bind_key.is_empty() {
        String::new()
    } else {
        bus.get_binding(&bind_key).unwrap_or_default()
    };
    let mut value = use_signal(|| initial_value);

    rsx! {
        textarea {
            class: "{classes}",
            placeholder: "{placeholder_text}",
            readonly: is_readonly,
            oninput: move |evt| {
                value.set(evt.value());
                if !bind_key.is_empty() {
                    bus.set_binding(&bind_key, &evt.value());
                }
            },
            "{value.read()}"
        }
    }
}

/// Render a Select element.
#[cfg(feature = "dioxus-ui")]
pub fn render_select(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let bind_key = element
        .props
        .get("bind")
        .or_else(|| element.props.get("binding"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let options_raw = element
        .props
        .get("options")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut bus = use_context::<ActionBus>();
    let initial_value = if bind_key.is_empty() {
        String::new()
    } else {
        bus.get_binding(&bind_key).unwrap_or_default()
    };

    rsx! {
        div { class: "w-full",
            select {
                class: "{classes}",
                onchange: move |evt| {
                    if !bind_key.is_empty() {
                        bus.set_binding(&bind_key, &evt.value());
                    }
                },
                option { value: "", "Select..." }
                for opt in options_raw.iter() {
                    option { value: "{opt}", selected: initial_value == *opt, "{opt}" }
                }
            }
        }
    }
}

/// Render an action-input element (bound input with signal).
#[cfg(feature = "dioxus-ui")]
pub fn render_action_input(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let bind_key = element
        .props
        .get("bind")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let placeholder_text = element
        .props
        .get("placeholder")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let input_type = element
        .props
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("text");

    let mut bus = use_context::<ActionBus>();
    let initial_value = bus.get_binding(&bind_key).unwrap_or_default();
    let mut value = use_signal(|| initial_value);

    rsx! {
        input {
            class: "{classes}",
            placeholder: "{placeholder_text}",
            r#type: "{input_type}",
            value: "{value.read()}",
            oninput: move |evt| {
                value.set(evt.value());
                bus.set_binding(&bind_key, &evt.value());
            }
        }
    }
}

/// Render an action-textarea element (bound textarea with signal).
#[cfg(feature = "dioxus-ui")]
pub fn render_action_textarea(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let bind_key = element
        .props
        .get("bind")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let placeholder_text = element
        .props
        .get("placeholder")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let is_readonly = element
        .props
        .get("readonly")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let mut bus = use_context::<ActionBus>();
    let initial_value = bus.get_binding(&bind_key).unwrap_or_default();
    let mut value = use_signal(|| initial_value);

    rsx! {
        textarea {
            class: "{classes}",
            placeholder: "{placeholder_text}",
            readonly: is_readonly,
            oninput: move |evt| {
                value.set(evt.value());
                bus.set_binding(&bind_key, &evt.value());
            },
            "{value.read()}"
        }
    }
}

/// Render an action-select element (bound select).
#[cfg(feature = "dioxus-ui")]
pub fn render_action_select(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let bind_key = element
        .props
        .get("bind")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let options_raw = element
        .props
        .get("options")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut bus = use_context::<ActionBus>();
    let initial_value = bus.get_binding(&bind_key).unwrap_or_default();

    rsx! {
        div { class: "w-full",
            select {
                class: "{classes}",
                onchange: move |evt| {
                    bus.set_binding(&bind_key, &evt.value());
                },
                option { value: "", "Select..." }
                for opt in options_raw.iter() {
                    option { value: "{opt}", selected: initial_value == *opt, "{opt}" }
                }
            }
        }
    }
}
