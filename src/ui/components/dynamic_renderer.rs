//! DynamicRenderer component
//!
//! Renders UI components dynamically based on component ID string.
//! This is the core of the Schema-driven UI (SDUI) system.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;

#[cfg(feature = "dioxus-ui")]
use crate::themes::ThemeVariant;

#[cfg(feature = "dioxus-ui")]
use crate::ui::flowbite_mapping::{resolve_flowbite_classes, resolve_layout_classes};

// Components extracted from this file — sibling modules in components/
#[cfg(feature = "dioxus-ui")]
use super::container_components::{render_badge, render_card, render_div, render_text};

#[cfg(feature = "dioxus-ui")]
use super::form_components::{
    render_action_input, render_action_select, render_action_textarea, render_input, render_select,
    render_textarea,
};

#[cfg(feature = "dioxus-ui")]
use super::interactive_components::{
    render_action_button, render_alert, render_button, render_checkbox, render_stepper,
    render_switch, render_tabs,
};

#[cfg(feature = "dioxus-ui")]
use super::modal::render_dialog;

#[cfg(feature = "dioxus-ui")]
use super::theme_provider::ThemeState;

/// Build the combined class string: Flowbite component classes + layout classes.
fn build_classes(element: &CanvasElement, theme_variant: ThemeVariant, is_dark: bool) -> String {
    let component = &element.component;
    let component_variant = element.variant.as_deref();
    let classes = resolve_flowbite_classes(
        component.as_str(),
        component_variant,
        theme_variant,
        is_dark,
    );
    let layout_classes = element
        .layout
        .as_ref()
        .map(resolve_layout_classes)
        .unwrap_or_default();
    if layout_classes.is_empty() {
        classes
    } else {
        format!("{classes} {layout_classes}")
    }
}

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DynamicRenderer(element: CanvasElement) -> Element {
    let ctx = use_context::<ThemeState>();
    let is_dark = ctx.is_dark();
    let theme_variant = *ctx.variant.read();

    if !element.visible {
        return rsx! { Fragment {} };
    }

    let component = &element.component;
    let classes = build_classes(&element, theme_variant, is_dark);

    match component.as_str() {
        "Div" | "div" => render_div(&element, &classes),
        "Text" | "text" => render_text(&element, &classes),
        "Card" | "card" => render_card(&element, &classes),
        "Badge" | "badge" => render_badge(&element, &classes),
        "Input" | "input" => render_input(&element, &classes, is_dark),
        "Textarea" | "textarea" => render_textarea(&element, &classes, is_dark),
        "Select" | "select" => render_select(&element, &classes, is_dark),
        "action-input" => render_action_input(&element, &classes, is_dark),
        "action-textarea" => render_action_textarea(&element, &classes, is_dark),
        "action-select" => render_action_select(&element, &classes, is_dark),
        "Button" | "button" => render_button(&element, &classes, is_dark),
        "action-button" => render_action_button(&element, &classes, is_dark),
        "alert" | "Alert" => render_alert(&element, &classes, is_dark),
        "switch" | "Switch" => render_switch(&element, &classes, is_dark),
        "checkbox" | "Checkbox" => render_checkbox(&element, &classes, is_dark),
        "tabs" | "Tabs" => render_tabs(&element, &classes, is_dark),
        "stepper" | "Stepper" => render_stepper(&element, &classes, is_dark),
        "Dialog" | "dialog" => render_dialog(&element, &classes, is_dark),
        _ => rsx! {
            div { class: "{classes}", "Unknown: {component}" }
        },
    }
}
