//! Modal and Dialog components
//!
//! Provides modal dialogs with overlay

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;
#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::modal as modal_classes;
#[cfg(feature = "dioxus-ui")]
use crate::ui::components::action_bus::{ActionBus, AppAction};
#[cfg(feature = "dioxus-ui")]
use crate::ui::components::theme_provider::ThemeState;
#[cfg(feature = "dioxus-ui")]
use crate::ui::css_var_utils::{bg_color, border_color, text_color};

/// Modal sizes
#[cfg(feature = "dioxus-ui")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Full,
}

#[cfg(feature = "dioxus-ui")]
impl ModalSize {
    pub fn as_classes(&self) -> &'static str {
        match self {
            ModalSize::Sm => "max-w-sm",
            ModalSize::Md => "max-w-md",
            ModalSize::Lg => "max-w-lg",
            ModalSize::Xl => "max-w-xl",
            ModalSize::Full => "max-w-full",
        }
    }
}

/// Dialog variants for SDUI rendering
#[cfg(feature = "dioxus-ui")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogVariant {
    #[default]
    Default,
    Destructive,
    Form,
}

#[cfg(feature = "dioxus-ui")]
impl DialogVariant {
    pub fn from_str_discrete(s: &str) -> Self {
        match s {
            "destructive" => DialogVariant::Destructive,
            "form" => DialogVariant::Form,
            _ => DialogVariant::Default,
        }
    }
}

/// Modal component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Modal(
    open: bool,
    title: String,
    size: ModalSize,
    class: String,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !open {
        return rsx! { {} };
    }

    let theme_state = use_context::<ThemeState>();
    let _is_dark = theme_state.is_dark();

    let size_classes = size.as_classes();

    // CSS variable values for theme-aware inline styles
    let bg_elevated = "var(--color-bg-elevated)";
    let border_col = "var(--color-border)";
    let text_primary = "var(--color-text-primary)";
    let text_tertiary = "var(--color-text-tertiary)";
    let _text_secondary = "var(--color-text-secondary)";

    // Build content classes - CSS variable patterns replaced with inline styles
    let content_class = format!(
        "{} {} w-full rounded-lg shadow-xl",
        size_classes,
        modal_classes::CONTENT
    );
    let content_bg_style = bg_color(bg_elevated);
    let content_border_style = border_color(border_col);

    let header_class = modal_classes::HEADER.to_string();
    let header_style = text_color(text_primary);

    // close_btn_class uses dark: and hover: which can't be inline - keep them but remove text-[var(...)]
    let close_btn_dark_class = "dark:hover:text-secondary";
    let close_btn_light_class = "hover:text-secondary";
    let close_btn_style = text_color(text_tertiary);

    rsx! {
        div {
            class: "{modal_classes::OVERLAY} bg-black bg-opacity-50",
            onclick: move |_| { on_close.call(()); },

            div {
                class: "{content_class} {class}",
                style: "{content_bg_style} {content_border_style}",
                onclick: move |evt| { evt.stop_propagation(); },

                div {
                    class: "{header_class}",
                    style: "{header_style}",

                    h3 {
                        class: "text-lg font-medium",
                        "{title}"
                    }

                    button {
                        class: "text-tertiary {close_btn_dark_class} {close_btn_light_class}",
                        style: "{close_btn_style}",
                        r#type: "button",
                        onclick: move |_| { on_close.call(()); },

                        svg {
                            class: "w-5 h-5",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                d: "M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
                            }
                        }
                    }
                }

                div {
                    class: "p-4 space-y-2",
                    {children}
                }

                div {
                    class: "flex items-center justify-end p-4 border-t space-x-2",
                    style: "{content_border_style}",
                }
            }
        }
    }
}

/// Dialog component (simpler modal with confirm/cancel)
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Dialog(
    open: bool,
    message: String,
    confirm_label: String,
    cancel_label: String,
    class: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    if !open {
        return rsx! { {} };
    }

    let theme_state = use_context::<ThemeState>();
    let is_dark = theme_state.is_dark();

    // CSS variable values for theme-aware inline styles
    let bg_elevated = "var(--color-bg-elevated)";
    let bg_surface = "var(--color-bg-surface)";
    let border_col = "var(--color-border)";
    let text_secondary = "var(--color-text-secondary)";
    let text_inverse = "var(--color-text-inverse)";
    let accent_primary = "var(--color-accent-primary)";
    let _accent_hover = "var(--color-accent-hover)";

    // Dialog content classes - CSS variable patterns replaced with inline styles
    let content_class = format!("max-w-sm w-full rounded-lg shadow-xl p-6 border {}", class);
    let content_bg_style = bg_color(bg_elevated);
    let content_border_style = border_color(border_col);

    let message_class = "mb-6".to_string();
    let message_style = text_color(text_secondary);

    // cancel_btn has hover states that can't be inline, so keep some classes but use inline for static colors
    let cancel_btn_class = "px-4 py-2 text-sm font-medium rounded-lg";
    let cancel_btn_hover_class = if is_dark {
        "dark:hover:bg-elevated"
    } else {
        "hover:bg-elevated"
    };
    let cancel_btn_style = format!(
        "{} {} {}",
        text_color(text_secondary),
        bg_color(bg_surface),
        border_color(border_col)
    );

    let confirm_btn_class = "px-4 py-2 text-sm font-medium rounded-lg";
    let confirm_btn_hover_class = "hover:bg-accent-hover";
    let confirm_btn_style = format!("{} {}", text_color(text_inverse), bg_color(accent_primary));

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
            onclick: move |_| { on_cancel.call(()); },

            div {
                class: "{content_class}",
                style: "{content_bg_style} {content_border_style}",
                onclick: move |evt| { evt.stop_propagation(); },

                p {
                    class: "{message_class}",
                    style: "{message_style}",
                    "{message}"
                }

                div {
                    class: "flex justify-end space-x-3",

                    button {
                        class: "{cancel_btn_class} {cancel_btn_hover_class}",
                        style: "{cancel_btn_style}",
                        r#type: "button",
                        onclick: move |_| { on_cancel.call(()); },

                        "{cancel_label}"
                    }

                    button {
                        class: "{confirm_btn_class} {confirm_btn_hover_class}",
                        style: "{confirm_btn_style}",
                        r#type: "button",
                        onclick: move |_| { on_confirm.call(()); },

                        "{confirm_label}"
                    }
                }
            }
        }
    }
}

/// Render a Dialog element for SDUI (Schema-Driven UI).
///
/// This function renders a dialog based on CanvasElement props:
/// - `title`: Dialog title text
/// - `message`: Dialog message text
/// - `confirm_label`: Confirm button label (default: "Confirm")
/// - `cancel_label`: Cancel button label (default: "Cancel")
/// - `variant`: Dialog variant - "default", "destructive", or "form" (default: "default")
#[cfg(feature = "dioxus-ui")]
pub fn render_dialog(element: &CanvasElement, classes: &str, is_dark: bool) -> Element {
    let title = element
        .props
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let message = element
        .props
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let confirm_label = element
        .props
        .get("confirm_label")
        .and_then(|v| v.as_str())
        .unwrap_or("Confirm")
        .to_string();
    let cancel_label = element
        .props
        .get("cancel_label")
        .and_then(|v| v.as_str())
        .unwrap_or("Cancel")
        .to_string();
    let variant_str = element
        .props
        .get("variant")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    let variant = DialogVariant::from_str_discrete(variant_str);

    let confirm_action = element
        .props
        .get("confirm_action")
        .and_then(|v| v.as_str())
        .unwrap_or("dialog_confirm")
        .to_string();
    let cancel_action = element
        .props
        .get("cancel_action")
        .and_then(|v| v.as_str())
        .unwrap_or("dialog_cancel")
        .to_string();

    let element_id = element.id.clone();
    let bus = use_context::<ActionBus>();

    let title_id = format!("{}-title", element_id);
    let message_id = format!("{}-message", element_id);

    let (confirm_btn_class, confirm_bg, confirm_text) = match variant {
        DialogVariant::Destructive => (
            "px-4 py-2 text-sm font-medium rounded-lg hover:bg-red-600",
            "bg-red-600",
            "text-white",
        ),
        DialogVariant::Form => (
            "px-4 py-2 text-sm font-medium rounded-lg hover:bg-accent-hover",
            "bg-accent-primary",
            "text-inverse",
        ),
        DialogVariant::Default => (
            "px-4 py-2 text-sm font-medium rounded-lg hover:bg-accent-hover",
            "bg-accent-primary",
            "text-inverse",
        ),
    };

    let cancel_btn_class = if is_dark {
        "px-4 py-2 text-sm font-medium rounded-lg dark:hover:bg-gray-700"
    } else {
        "px-4 py-2 text-sm font-medium rounded-lg hover:bg-gray-100"
    };

    let overlay_class =
        "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50";
    let content_class = format!(
        "max-w-md w-full rounded-lg shadow-xl p-6 border {}",
        classes
    );
    let content_bg = bg_color("var(--color-bg-elevated)");
    let content_border = border_color("var(--color-border)");
    let message_style = text_color("var(--color-text-secondary)");
    let title_style = text_color("var(--color-text-primary)");
    let cancel_btn_style = format!(
        "{} {} {}",
        text_color("var(--color-text-secondary)"),
        bg_color("var(--color-bg-surface)"),
        border_color("var(--color-border)")
    );
    let confirm_btn_style = format!("background-color: {}; color: {};", confirm_bg, confirm_text);
    let bus_cancel = bus.clone();
    let bus_confirm = bus.clone();
    let cancel_action_final = cancel_action.clone();
    let confirm_action_final = confirm_action.clone();
    let element_id_for_cancel = element_id.clone();
    let element_id_for_confirm = element_id.clone();

    rsx! {
        div {
            class: "{overlay_class}",
            onclick: move |_evt| {},

            div {
                class: "{content_class}",
                style: "{content_bg} {content_border}",
                onclick: move |evt| { evt.stop_propagation(); },
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "{title_id}",
                aria_describedby: "{message_id}",

                if !title.is_empty() {
                    h3 {
                        id: "{title_id}",
                        class: "text-lg font-medium mb-4",
                        style: "{title_style}",
                        "{title}"
                    }
                }

                p {
                    id: "{message_id}",
                    class: "mb-6 text-sm",
                    style: "{message_style}",
                    "{message}"
                }

                div {
                    class: "flex justify-end space-x-3",

                    button {
                        class: "{cancel_btn_class}",
                        style: "{cancel_btn_style}",
                        r#type: "button",
                        onclick: move |_| {
                            let mut bus = bus_cancel.clone();
                            bus.dispatch(AppAction {
                                name: cancel_action_final.clone(),
                                source: element_id_for_cancel.clone(),
                                payload: None,
                            });
                        },

                        "{cancel_label}"
                    }

                    button {
                        class: "{confirm_btn_class}",
                        style: "{confirm_btn_style}",
                        r#type: "button",
                        onclick: move |_| {
                            let mut bus = bus_confirm.clone();
                            bus.dispatch(AppAction {
                                name: confirm_action_final.clone(),
                                source: element_id_for_confirm.clone(),
                                payload: None,
                            });
                        },

                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
