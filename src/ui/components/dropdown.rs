//! Dropdown Menu components
//!
//! Provides a dropdown menu with trigger, items, dividers, and overlay rendering.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::ui::components::theme_provider::ThemeState;
#[cfg(feature = "dioxus-ui")]
use crate::ui::css_var_utils::{bg_color, border_color, text_color};

/// Position where the dropdown menu should appear relative to the trigger.
#[cfg(feature = "dioxus-ui")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownPosition {
    #[default]
    BottomStart,
    BottomEnd,
    TopStart,
    TopEnd,
}

/// Properties for a dropdown menu item.
#[cfg(feature = "dioxus-ui")]
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
    /// Label text for the menu item.
    pub label: String,
    /// Optional icon name (Material Symbols).
    pub icon: Option<String>,
    /// Optional keyboard shortcut display text.
    pub shortcut: Option<String>,
    /// Whether the item is disabled.
    pub disabled: bool,
    /// Whether the item is destructive (red styling).
    pub destructive: bool,
}

/// Dropdown Menu component with overlay rendering.
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Dropdown(
    /// Whether the dropdown is open.
    open: bool,
    /// Position of the menu relative to the trigger.
    position: DropdownPosition,
    /// Class for the trigger button.
    trigger_class: String,
    /// Class for the menu container.
    menu_class: String,
    /// Event handler when dropdown closes.
    on_close: EventHandler<()>,
    /// The trigger element (button/link) that opens the menu.
    trigger: Element,
    /// Child elements (DropdownItem or DropdownDivider).
    children: Element,
) -> Element {
    if !open {
        return rsx! { {trigger} };
    }

    let theme_state = use_context::<ThemeState>();
    let _is_dark = theme_state.is_dark();

    let bg_elevated = "var(--color-bg-elevated)";
    let border_col = "var(--color-border)";

    let position_class = match position {
        DropdownPosition::BottomStart => "origin-top-left left-0 top-full mt-1",
        DropdownPosition::BottomEnd => "origin-top-right right-0 top-full mt-1",
        DropdownPosition::TopStart => "origin-bottom-left left-0 bottom-full mb-1",
        DropdownPosition::TopEnd => "origin-bottom-right right-0 bottom-full mb-1",
    };

    let menu_style = format!(
        "{} {} {}",
        bg_color(bg_elevated),
        border_color(border_col),
        "min-w-48"
    );

    rsx! {
        {trigger}

        div {
            class: "fixed inset-0 z-50",
            onclick: move |_| { on_close.call(()); },

            div {
                class: "{position_class} z-50 {menu_class}",
                style: "{menu_style}",
                onclick: move |evt| { evt.stop_propagation(); },

                div {
                    class: "py-1 rounded-md shadow-lg ring-1 ring-black/5 dark:ring-white/10 overflow-hidden",
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_label: "Dropdown menu",

                    {children}
                }
            }
        }
    }
}

/// A single item in the dropdown menu.
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DropdownItem(
    /// Label text.
    label: String,
    /// Icon name (Material Symbols).
    icon: Option<String>,
    /// Keyboard shortcut display.
    shortcut: Option<String>,
    /// Whether the item is disabled.
    disabled: bool,
    /// Whether the item is destructive.
    destructive: bool,
    /// Click handler.
    on_click: EventHandler<()>,
) -> Element {
    let theme_state = use_context::<ThemeState>();
    let _is_dark = theme_state.is_dark();

    let text_primary = "var(--color-text-primary)";
    let text_secondary = "var(--color-text-secondary)";
    let text_disabled = "var(--color-text-disabled)";
    let danger = "var(--color-danger)";
    let bg_hover = "var(--color-bg-hover)";

    let (text_style, hover_style, icon_color): (String, String, String) = if disabled {
        (
            text_color(text_disabled),
            String::new(),
            text_disabled.to_string(),
        )
    } else if destructive {
        (text_color(danger), bg_color(bg_hover), danger.to_string())
    } else {
        (
            text_color(text_primary),
            bg_color(bg_hover),
            text_secondary.to_string(),
        )
    };

    let item_class = if disabled {
        "flex items-center justify-between w-full px-4 py-2 text-sm cursor-not-allowed opacity-50"
            .to_string()
    } else {
        "flex items-center justify-between w-full px-4 py-2 text-sm cursor-pointer transition-colors".to_string()
    };

    rsx! {
        button {
            class: "{item_class} {hover_style}",
            style: "{text_style}",
            disabled: disabled,
            role: "menuitem",
            onclick: move |_| {
                if !disabled {
                    on_click.call(());
                }
            },

            span {
                class: "flex items-center gap-2",

                if let Some(icon_name) = icon {
                    span {
                        class: "material-symbols-rounded text-lg",
                        style: "color: {icon_color};",
                        "{icon_name}"
                    }
                }

                span {
                    class: "font-medium",
                    "{label}"
                }
            }

            if let Some(shortcut_text) = shortcut {
                span {
                    class: "text-xs opacity-60",
                    "{shortcut_text}"
                }
            }
        }
    }
}

/// A divider between menu items.
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DropdownDivider() -> Element {
    let border_col = "var(--color-border)";

    rsx! {
        div {
            class: "my-1 border-t",
            style: border_color(border_col),
            role: "separator"
        }
    }
}

/// Helper component to render dropdown items from a list of props.
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn DropdownItems(items: Vec<DropdownMenuItemProps>, on_select: EventHandler<usize>) -> Element {
    rsx! {
        for (index, item) in items.iter().enumerate() {
            if item.label == "---" {
                DropdownDivider {}
            } else {
                DropdownItem {
                    key: "{index}",
                    label: item.label.clone(),
                    icon: item.icon.clone(),
                    shortcut: item.shortcut.clone(),
                    disabled: item.disabled,
                    destructive: item.destructive,
                    on_click: move |_| { on_select.call(index); }
                }
            }
        }
    }
}
