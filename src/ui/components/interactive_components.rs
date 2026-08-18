//! Interactive components for SDUI — Button, Alert, Switch, Checkbox, Tabs, Stepper.
//!
//! Extracted from dynamic_renderer.rs to reduce file size and improve maintainability.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;

#[cfg(feature = "dioxus-ui")]
use super::action_bus::{ActionBus, AppAction};

/// Render a Button element.
#[cfg(feature = "dioxus-ui")]
pub fn render_button(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let label_text = element
        .props
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Button");
    let route = element
        .props
        .get("route")
        .and_then(|v| v.as_str())
        .map(String::from);
    let action_name = element
        .props
        .get("action")
        .and_then(|v| v.as_str())
        .map(String::from);
    let modal_id = element
        .props
        .get("modal")
        .and_then(|v| v.as_str())
        .map(String::from);
    let element_id = element.id.clone();
    let mut bus = use_context::<ActionBus>();

    rsx! {
        button {
            class: "{classes}",
            onclick: move |_| {
                if let Some(route) = &route {
                    bus.navigate(route, None);
                } else if let Some(modal) = &modal_id {
                    bus.open_modal(modal);
                } else if let Some(name) = &action_name {
                    bus.dispatch(AppAction {
                        name: name.clone(),
                        source: element_id.clone(),
                        payload: None,
                    });
                }
            },
            "{label_text}"
        }
    }
}

/// Render an action-button element (dispatches action on click).
#[cfg(feature = "dioxus-ui")]
pub fn render_action_button(element: &CanvasElement, classes: &str, _is_dark: bool) -> Element {
    let label_text = element
        .props
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Button");
    let action_name = element
        .props
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let element_id = element.id.clone();
    let mut bus = use_context::<ActionBus>();

    rsx! {
        button {
            class: "{classes}",
            onclick: move |_| {
                let action = AppAction {
                    name: action_name.clone(),
                    source: element_id.clone(),
                    payload: None,
                };
                bus.dispatch(action);
            },
            "{label_text}"
        }
    }
}

/// Render an Alert element.
#[cfg(feature = "dioxus-ui")]
pub fn render_alert(element: &CanvasElement, _classes: &str, _is_dark: bool) -> Element {
    let message_text = element
        .props
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let alert_variant = element
        .props
        .get("variant")
        .and_then(|v| v.as_str())
        .unwrap_or("info");
    let is_dismissible = element
        .props
        .get("dismissible")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let element_id = element.id.clone();
    let mut bus = use_context::<ActionBus>();

    let (bg_class, border_class, icon_class, icon_name) = match alert_variant {
        "success" => (
            "bg-emerald-50 dark:bg-emerald-900/30",
            "border-emerald-200 dark:border-emerald-700",
            "text-emerald-600 dark:text-emerald-400",
            "check_circle",
        ),
        "warning" => (
            "bg-amber-50 dark:bg-amber-900/30",
            "border-amber-200 dark:border-amber-700",
            "text-amber-600 dark:text-amber-400",
            "warning",
        ),
        "error" => (
            "bg-red-50 dark:bg-red-900/30",
            "border-red-200 dark:border-red-700",
            "text-red-600 dark:text-red-400",
            "error",
        ),
        _ => (
            // info
            "bg-blue-50 dark:bg-blue-900/30",
            "border-blue-200 dark:border-blue-700",
            "text-blue-600 dark:text-blue-400",
            "info",
        ),
    };

    rsx! {
        div {
            class: "flex items-start gap-3 p-4 rounded-lg border {bg_class} {border_class}",
            role: "alert",
            if !icon_name.is_empty() {
                span {
                    class: "material-symbols-rounded {icon_class} text-xl flex-shrink-0",
                    "{icon_name}"
                }
            }
            div { class: "flex-1 text-sm text-gray-700 dark:text-gray-200", "{message_text}" }
            if is_dismissible {
                button {
                    class: "ml-auto p-1 rounded hover:bg-black/5 dark:hover:bg-white/10 {icon_class}",
                    onclick: move |_| {
                        bus.dispatch(AppAction {
                            name: element_id.clone(),
                            source: element_id.clone(),
                            payload: None,
                        });
                    },
                    span { class: "material-symbols-rounded text-lg", "close" }
                }
            }
        }
    }
}

/// Render a Switch (toggle) element.
#[cfg(feature = "dioxus-ui")]
pub fn render_switch(element: &CanvasElement, _classes: &str, is_dark: bool) -> Element {
    let label_text = element
        .props
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let bind_key = element
        .props
        .get("bind")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let element_id = element.id.clone();
    let mut bus = use_context::<ActionBus>();

    let initial_checked = if bind_key.is_empty() {
        element
            .props
            .get("checked")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    } else {
        bus.get_binding(&bind_key)
            .map(|v| v == "true")
            .unwrap_or(false)
    };
    let mut is_checked = use_signal(|| initial_checked);

    let track_class = if *is_checked.read() {
        "bg-indigo-600"
    } else {
        if is_dark {
            "bg-neutral-600"
        } else {
            "bg-gray-200"
        }
    };
    let thumb_class = if *is_checked.read() {
        "translate-x-5"
    } else {
        "translate-x-1"
    };

    rsx! {
        label {
            class: "inline-flex items-center gap-3 cursor-pointer",
            div {
                class: "relative inline-flex h-6 w-11 items-center rounded-full transition-colors {track_class}",
                input {
                    r#type: "checkbox",
                    class: "sr-only",
                    checked: *is_checked.read(),
                    onchange: move |evt| {
                        let new_val = evt.value().parse().unwrap_or(false);
                        is_checked.set(new_val);
                        if !bind_key.is_empty() {
                            bus.set_binding(&bind_key, &new_val.to_string());
                        }
                        bus.dispatch(AppAction {
                            name: "switch_toggle".to_string(),
                            source: element_id.clone(),
                            payload: Some(serde_json::Value::String(new_val.to_string())),
                        });
                    }
                }
                div {
                    class: "inline-block h-5 w-5 transform rounded-full bg-white shadow-md transition-transform {thumb_class}"
                }
            }
            span { class: "text-sm text-gray-700 dark:text-gray-200", "{label_text}" }
        }
    }
}

/// Render a Checkbox element.
#[cfg(feature = "dioxus-ui")]
pub fn render_checkbox(element: &CanvasElement, _classes: &str, _is_dark: bool) -> Element {
    let label_text = element
        .props
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let bind_key = element
        .props
        .get("bind")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let element_id = element.id.clone();
    let mut bus = use_context::<ActionBus>();

    let initial_checked = if bind_key.is_empty() {
        element
            .props
            .get("checked")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    } else {
        bus.get_binding(&bind_key)
            .map(|v| v == "true")
            .unwrap_or(false)
    };
    let mut is_checked = use_signal(|| initial_checked);

    rsx! {
        label {
            class: "inline-flex items-center gap-3 cursor-pointer",
            input {
                r#type: "checkbox",
                class: "w-4 h-4 text-indigo-600 rounded border-gray-300 dark:border-neutral-600 focus:ring-indigo-500",
                checked: *is_checked.read(),
                onchange: move |evt| {
                    let new_val = evt.value().parse().unwrap_or(false);
                    is_checked.set(new_val);
                    if !bind_key.is_empty() {
                        bus.set_binding(&bind_key, &new_val.to_string());
                    }
                    bus.dispatch(AppAction {
                        name: "checkbox_change".to_string(),
                        source: element_id.clone(),
                        payload: Some(serde_json::Value::String(new_val.to_string())),
                    });
                }
            }
            span { class: "text-sm text-gray-700 dark:text-gray-200", "{label_text}" }
        }
    }
}

/// Render a Tabs element.
#[cfg(feature = "dioxus-ui")]
pub fn render_tabs(element: &CanvasElement, _classes: &str, _is_dark: bool) -> Element {
    let options_raw = element
        .props
        .get("options")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|v| {
                    let label = v
                        .get("label")
                        .and_then(|l| l.as_str())
                        .unwrap_or("")
                        .to_string();
                    let active = v.get("active").and_then(|a| a.as_bool()).unwrap_or(false);
                    (label, active)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut active_index = use_signal(|| options_raw.iter().position(|(_, a)| *a).unwrap_or(0));

    let active_label = options_raw
        .get(*active_index.read())
        .map(|(l, _)| l.clone())
        .unwrap_or_default();

    rsx! {
        div {
            div {
                class: "flex border-b border-gray-200 dark:border-neutral-700",
                for (i, (tab_label, _)) in options_raw.iter().enumerate() {
                    button {
                        class: if i == *active_index.read() {
                            "px-4 py-2 text-sm font-medium border-b-2 border-indigo-600 text-indigo-600 dark:text-indigo-400 dark:border-indigo-400"
                        } else {
                            "px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                        },
                        onclick: move |_| { active_index.set(i); },
                        "{tab_label}"
                    }
                }
            }
            div {
                class: "p-4 text-sm text-gray-600 dark:text-gray-300",
                "{active_label}"
            }
        }
    }
}

/// Render a Stepper (progress steps) element.
#[cfg(feature = "dioxus-ui")]
pub fn render_stepper(element: &CanvasElement, _classes: &str, _is_dark: bool) -> Element {
    let current_step = element
        .props
        .get("step")
        .and_then(|v| v.as_i64())
        .unwrap_or(1) as usize;
    let total_steps = element
        .props
        .get("total")
        .and_then(|v| v.as_i64())
        .unwrap_or(3) as usize;
    let labels_raw = element
        .props
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let safe_total = if total_steps == 0 { 1 } else { total_steps };

    rsx! {
        div {
            class: "flex items-center justify-between w-full",
            for step_idx in 1..=safe_total {
                div {
                    class: "flex flex-col items-center gap-1",
                    if step_idx < current_step {
                        div {
                            class: "w-8 h-8 rounded-full bg-indigo-600 flex items-center justify-center text-white text-sm font-medium",
                            span { class: "material-symbols-rounded text-lg", "check" }
                        }
                    } else if step_idx == current_step {
                        div {
                            class: "w-8 h-8 rounded-full bg-indigo-600 flex items-center justify-center text-white text-sm font-medium ring-4 ring-indigo-200 dark:ring-indigo-900",
                            "{step_idx}"
                        }
                    } else {
                        div {
                            class: "w-8 h-8 rounded-full bg-gray-200 dark:bg-neutral-700 flex items-center justify-center text-gray-500 dark:text-gray-400 text-sm font-medium",
                            "{step_idx}"
                        }
                    }
                    if let Some(label) = labels_raw.get(step_idx - 1) {
                        span { class: "text-xs text-gray-500 dark:text-gray-400 text-center", "{label}" }
                    }
                }
                if step_idx < safe_total {
                    div {
                        class: if step_idx < current_step {
                            "flex-1 h-0.5 bg-indigo-600"
                        } else {
                            "flex-1 h-0.5 bg-gray-200 dark:bg-neutral-700"
                        }
                    }
                }
            }
        }
    }
}
