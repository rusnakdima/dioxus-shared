//! DynamicPage component
//!
//! Schema-driven page component that renders a complete page from a Schema.
//! This is the main entry point for SDUI pages.
//!
//! Features:
//! - Renders page elements from schema
//! - Theme provided via ThemeState context (no dark_mode prop needed)
//! - Registers keyboard shortcuts from schema.shortcuts
//! - Supports modal dialogs via ActionBus

#[cfg(feature = "dioxus-ui")]
use super::action_bus::ActionBus;
#[cfg(feature = "dioxus-ui")]
use crate::schema::{Modal as SchemaModal, Schema};
#[cfg(feature = "dioxus-ui")]
use crate::shortcuts::{register_shortcuts, ShortcutDef};
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DynamicPage(schema: Schema, initial_route: String, bus: ActionBus) -> Element {
    let current_route = bus.current_route();
    let current_modal_signal = bus.current_modal;

    use_effect(move || {
        let shortcut_defs: Vec<ShortcutDef> = schema
            .shortcuts
            .iter()
            .map(|s| ShortcutDef {
                id: s.id.clone(),
                keys: s.keys.clone(),
                action: s.action.clone(),
            })
            .collect();
        register_shortcuts(&shortcut_defs);
    });

    let page = schema
        .pages
        .iter()
        .find(|p| p.route == current_route)
        .or_else(|| schema.pages.first());

    let get_modal_content = |modal_id: &str| -> Option<&SchemaModal> {
        schema.modals.iter().find(|m| m.id == modal_id)
    };

    let modal_element = if let Some(modal_id) = current_modal_signal.read().as_deref() {
        get_modal_content(modal_id)
            .map(|modal| {
                rsx! {
                    ModalOverlay {
                        modal: modal.clone(),
                        bus: bus.clone()
                    }
                }
            })
            .transpose()?
    } else {
        None
    };

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900",

            div {
                class: "container mx-auto px-4 py-6",

                if let Some(page) = page {
                    div {
                        class: "flex flex-col gap-4",

                        for element in page.elements.iter().filter(|e| e.visible) {
                            crate::ui::components::dynamic_renderer::DynamicRenderer {
                                element: element.clone()
                            }
                        }
                    }
                } else {
                    div {
                        class: "text-center text-gray-500 dark:text-gray-400 py-10",
                        "No page found for route: {current_route}"
                    }
                }
            }

            {modal_element}
        }
    }
}

#[cfg(feature = "dioxus-ui")]
#[component]
fn ModalOverlay(modal: SchemaModal, bus: ActionBus) -> Element {
    let mut bus_close = bus.clone();

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 flex items-center justify-center z-50",
            onclick: move |_| { bus.close_modal(); },

            div {
                class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4",
                onclick: move |e| { e.stop_propagation(); },

                div {
                    class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",

                    h2 {
                        class: "text-lg font-medium text-gray-900 dark:text-white",
                        "{modal.title}"
                    }

                    button {
                        class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                        onclick: move |_| { bus_close.close_modal(); },
                        "\u{2715}"
                    }
                }

                div {
                    class: "p-4 space-y-2",

                    for elem in modal.elements.iter().filter(|e| e.visible) {
                        crate::ui::components::dynamic_renderer::DynamicRenderer {
                            element: elem.clone()
                        }
                    }
                }
            }
        }
    }
}
