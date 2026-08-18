//! Container components for SDUI — Div, Text, Card, Badge.
//!
//! Extracted from dynamic_renderer.rs to reduce file size and improve maintainability.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;

#[cfg(feature = "dioxus-ui")]
use super::super::DynamicRenderer;

/// Render a Div (container) element — renders children.
#[cfg(feature = "dioxus-ui")]
pub fn render_div(element: &CanvasElement, classes: &str) -> Element {
    rsx! {
        div { class: "{classes}",
            for child in element.children.iter() {
                DynamicRenderer { element: child.clone() }
            }
        }
    }
}

/// Render a Text element.
#[cfg(feature = "dioxus-ui")]
pub fn render_text(element: &CanvasElement, classes: &str) -> Element {
    let text = element
        .props
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    rsx! {
        span { class: "{classes}", "{text}" }
    }
}

/// Render a Card element — renders children in a card container.
#[cfg(feature = "dioxus-ui")]
pub fn render_card(element: &CanvasElement, classes: &str) -> Element {
    rsx! {
        div { class: "{classes}",
            for child in element.children.iter() {
                DynamicRenderer { element: child.clone() }
            }
        }
    }
}

/// Render a Badge element.
#[cfg(feature = "dioxus-ui")]
pub fn render_badge(element: &CanvasElement, classes: &str) -> Element {
    let label_text = element
        .props
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    rsx! {
        span { class: "{classes}", "{label_text}" }
    }
}
