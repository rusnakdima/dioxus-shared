//! DynamicRenderer component
//!
//! Renders UI components dynamically based on component ID string.
//! This is the core of the Schema-driven UI (SDUI) system.
//!
//! Uses ThemeState context to determine variant and dark/light mode.
//! Classes are mapped through ClassMapper to support semantic names.
//!
//! Interactive components (action-* variants) dispatch actions to ActionBus context.

#[cfg(feature = "dioxus-ui")]
use std::collections::HashMap;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;
#[cfg(feature = "dioxus-ui")]
use super::action_bus::{ActionBus, AppAction};
#[cfg(feature = "dioxus-ui")]
use super::class_mapper::ClassMapper;
#[cfg(feature = "dioxus-ui")]
use super::theme_provider::ThemeState;

#[cfg(feature = "dioxus-ui")]
fn merge_classes(base: &str, override_: &str) -> String {
    let mut classes = HashMap::new();
    for cls in base.split_whitespace() {
        let key = cls.split('-').next().unwrap_or(cls);
        classes.insert(key.to_string(), cls.to_string());
    }
    for cls in override_.split_whitespace() {
        let key = cls.split('-').next().unwrap_or(cls);
        classes.insert(key.to_string(), cls.to_string());
    }
    classes.into_values().collect::<Vec<_>>().join(" ")
}

#[cfg(feature = "dioxus-ui")]
fn get_theme_classes(classes: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    ClassMapper::merge_with_defaults(classes, _variant, is_dark)
}

#[cfg(feature = "dioxus-ui")]
fn get_input_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark {
        "w-full px-4 py-3 rounded-2xl border border-neutral-600 bg-neutral-800 text-white placeholder-neutral-400"
    } else {
        "w-full px-4 py-3 rounded-2xl border border-neutral-300 bg-white text-gray-900 placeholder-gray-400"
    };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
fn get_surface_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark {
        "bg-neutral-900 text-white border-neutral-700"
    } else {
        "bg-white text-gray-900 border-neutral-200"
    };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
fn get_text_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark { "text-white" } else { "text-gray-900" };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
fn get_btn_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark {
        "bg-neutral-700 hover:bg-neutral-600 text-white px-5 py-2.5 rounded-2xl font-medium transition-colors"
    } else {
        "bg-indigo-600 hover:bg-indigo-700 text-white px-5 py-2.5 rounded-2xl font-medium transition-colors"
    };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
fn get_card_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark {
        "bg-neutral-800 rounded-3xl shadow-sm border border-neutral-700 p-4"
    } else {
        "bg-white rounded-3xl shadow-sm border border-neutral-100 p-4"
    };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
fn get_badge_classes(extra: &str, ctx: &ThemeState) -> String {
    let _variant = *ctx.variant.read();
    let is_dark = ctx.is_dark();
    let base = if is_dark {
        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-900 text-indigo-200"
    } else {
        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800"
    };
    merge_classes(base, extra)
}

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DynamicRenderer(element: CanvasElement) -> Element {
    let ctx = use_context::<ThemeState>();
    let component = &element.component;
    let classes = &element.classes;

    if !element.visible {
        return rsx! { Fragment {} };
    }

    let merged_classes = get_theme_classes(classes, &ctx);

    match component.as_str() {
        "Div" | "div" => {
            rsx! {
                div { class: "{merged_classes}",
                    for child in element.children.iter() {
                        DynamicRenderer { element: child.clone() }
                    }
                }
            }
        }

        "Text" | "text" => {
            let text = element.props.get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let text_class = get_text_classes(classes, &ctx);
            rsx! {
                span { class: "{text_class}", "{text}" }
            }
        }

        "Button" | "button" => {
            let label_text = element.props.get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("Button");
            let btn_class = get_btn_classes(classes, &ctx);

            let route = element.props.get("route")
                .and_then(|v| v.as_str())
                .map(String::from);
            let action_name = element.props.get("action")
                .and_then(|v| v.as_str())
                .map(String::from);
            let modal_id = element.props.get("modal")
                .and_then(|v| v.as_str())
                .map(String::from);
            let element_id = element.id.clone();
            let mut bus = use_context::<ActionBus>();

            rsx! {
                button {
                    class: "{btn_class}",
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

        "Select" | "select" => {
            let bind_key = element.props.get("bind")
                .or_else(|| element.props.get("binding"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let options_raw = element.props.get("options")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let select_class = get_input_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            let initial_value = if bind_key.is_empty() {
                String::new()
            } else {
                bus.get_binding(&bind_key).unwrap_or_default()
            };

            rsx! {
                div { class: "w-full",
                    select {
                        class: "{select_class}",
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

        "Textarea" | "textarea" => {
            let bind_key = element.props.get("bind")
                .or_else(|| element.props.get("binding"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let placeholder_text = element.props.get("placeholder")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let is_readonly = element.props.get("readonly")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let textarea_class = get_input_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            let initial_value = if bind_key.is_empty() {
                String::new()
            } else {
                bus.get_binding(&bind_key).unwrap_or_default()
            };
            let mut value = use_signal(|| initial_value.clone());

            rsx! {
                textarea {
                    class: "{textarea_class}",
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

        "Input" | "input" => {
            let placeholder_text = element.props.get("placeholder")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let input_class = get_input_classes(classes, &ctx);

            rsx! {
                input {
                    class: "{input_class}",
                    placeholder: "{placeholder_text}",
                    r#type: "text"
                }
            }
        }

        "Card" | "card" => {
            let card_class = get_card_classes(classes, &ctx);
            rsx! {
                div { class: "{card_class}",
                    for child in element.children.iter() {
                        DynamicRenderer { element: child.clone() }
                    }
                }
            }
        }

        "Badge" | "badge" => {
            let label_text = element.props.get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let badge_class = get_badge_classes(classes, &ctx);

            rsx! {
                span { class: "{badge_class}", "{label_text}" }
            }
        }

        "action-button" => {
            let label_text = element.props.get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("Button");
            let action_name = element.props.get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let element_id = element.id.clone();
            let btn_class = get_btn_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            rsx! {
                button {
                    class: "{btn_class}",
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

        "action-select" => {
            let bind_key = element.props.get("bind")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let options_raw = element.props.get("options")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let select_class = get_input_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            let initial_value = bus.get_binding(&bind_key).unwrap_or_default();

            rsx! {
                div { class: "w-full",
                    select {
                        class: "{select_class}",
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

        "action-textarea" => {
            let bind_key = element.props.get("bind")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let placeholder_text = element.props.get("placeholder")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let is_readonly = element.props.get("readonly")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let textarea_class = get_input_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            let initial_value = bus.get_binding(&bind_key).unwrap_or_default();
            let mut value = use_signal(|| initial_value);

            rsx! {
                textarea {
                    class: "{textarea_class}",
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

        "action-input" => {
            let bind_key = element.props.get("bind")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let placeholder_text = element.props.get("placeholder")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let input_type = element.props.get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("text");
            let input_class = get_input_classes(classes, &ctx);

            let mut bus = use_context::<ActionBus>();
            let initial_value = bus.get_binding(&bind_key).unwrap_or_default();
            let mut value = use_signal(|| initial_value);

            rsx! {
                input {
                    class: "{input_class}",
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

        _ => {
            let fallback_class = get_surface_classes(classes, &ctx);
            rsx! {
                div { class: "{fallback_class}", "Unknown: {component}" }
            }
        }
    }
}
