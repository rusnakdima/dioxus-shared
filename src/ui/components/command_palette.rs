//! Command Palette component
//!
//! A keyboard-driven command palette with search, grouped results, and keyboard navigation.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::ui::components::theme_provider::ThemeState;

// ============================================================================
// Data Types
// ============================================================================

/// A single command or result item
#[derive(Debug, Clone, PartialEq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
    pub disabled: bool,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            shortcut: None,
            icon: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A group of commands
#[derive(Debug, Clone, PartialEq)]
pub struct CommandGroup {
    pub name: String,
    pub commands: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(name: impl Into<String>, commands: Vec<CommandItem>) -> Self {
        Self {
            name: name.into(),
            commands,
        }
    }
}

// ============================================================================
// CommandItemRow Component
// ============================================================================

#[cfg(feature = "dioxus-ui")]
#[derive(Props, Clone, PartialEq)]
struct CommandPaletteItemRowProps {
    item: CommandItem,
    is_selected: bool,
    search_query: String,
    on_select: EventHandler<String>,
}

#[cfg(feature = "dioxus-ui")]
#[component]
fn CommandPaletteItemRow(props: CommandPaletteItemRowProps) -> Element {
    let text_primary = "var(--color-text-primary)";
    let text_secondary = "var(--color-text-secondary)";
    let text_muted = "var(--color-text-tertiary)";
    let bg_elevated = "var(--color-bg-elevated)";
    let accent_primary = "var(--color-accent-primary)";

    let item_style = if props.is_selected {
        format!(
            "background-color: {}; border-color: {};",
            bg_elevated, accent_primary
        )
    } else {
        String::new()
    };

    let item_class = if props.is_selected {
        "flex items-center w-full px-4 py-3 text-left rounded-lg border cursor-pointer transition-colors"
    } else {
        "flex items-center w-full px-4 py-3 text-left rounded-lg cursor-pointer transition-colors"
    };

    let label_content = if props.search_query.is_empty() {
        rsx! { "{props.item.label}" }
    } else {
        let query_lower = props.search_query.to_lowercase();
        let label_lower = props.item.label.to_lowercase();
        if let Some(pos) = label_lower.find(&query_lower) {
            let before = &props.item.label[..pos];
            let match_text = &props.item.label[pos..pos + props.search_query.len()];
            let after = &props.item.label[pos + props.search_query.len()..];
            rsx! {
                "{before}"
                span {
                    style: "font-weight: 600; color: var(--color-accent-primary);",
                    "{match_text}"
                }
                "{after}"
            }
        } else {
            rsx! { "{props.item.label}" }
        }
    };

    let item_id = props.item.id.clone();
    let item_disabled = props.item.disabled;
    let on_select = props.on_select;

    rsx! {
        button {
            class: "{item_class}",
            style: "{item_style}",
            disabled: item_disabled,
            onclick: move |_| {
                if !item_disabled {
                    on_select.call(item_id.clone());
                }
            },
            div {
                class: "flex-1 min-w-0",
                div {
                    class: "flex items-center justify-between",
                    div {
                        class: "flex-1 min-w-0",
                        style: "color: {text_primary};",
                        {label_content}
                    }
                    if let Some(ref shortcut) = props.item.shortcut {
                        div {
                            class: "ml-2 flex-shrink-0",
                            style: "color: {text_muted};",
                            span {
                                class: "text-xs font-mono px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700",
                                "{shortcut}"
                            }
                        }
                    }
                }
                if let Some(ref description) = props.item.description {
                    div {
                        class: "text-sm truncate mt-0.5",
                        style: "color: {text_secondary};",
                        "{description}"
                    }
                }
            }
        }
    }
}

// ============================================================================
// CommandGroupSection Component
// ============================================================================

#[cfg(feature = "dioxus-ui")]
#[derive(Props, Clone, PartialEq)]
struct CommandGroupSectionProps {
    group: CommandGroup,
    selected_id: Option<String>,
    search_query: String,
    on_select: EventHandler<String>,
}

#[cfg(feature = "dioxus-ui")]
#[component]
fn CommandGroupSection(props: CommandGroupSectionProps) -> Element {
    let text_muted = "var(--color-text-tertiary)";

    let filtered_commands: Vec<CommandItem> = if props.search_query.is_empty() {
        props.group.commands.clone()
    } else {
        let query_lower = props.search_query.to_lowercase();
        props
            .group
            .commands
            .iter()
            .filter(|cmd| {
                cmd.label.to_lowercase().contains(&query_lower)
                    || cmd
                        .description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    };

    if filtered_commands.is_empty() {
        return rsx! { {} };
    }

    let on_select = props.on_select;
    let selected_id = props.selected_id.clone();
    let search_query = props.search_query.clone();

    rsx! {
        div {
            class: "mb-2",
            div {
                class: "px-4 py-2 text-xs font-semibold uppercase tracking-wider",
                style: "color: {text_muted};",
                "{props.group.name}"
            }
            div {
                class: "space-y-1",
                for (idx, item) in filtered_commands.iter().enumerate() {
                    CommandPaletteItemRow {
                        key: "{idx}",
                        item: item.clone(),
                        is_selected: selected_id.as_ref() == Some(&item.id),
                        search_query: search_query.clone(),
                        on_select: on_select,
                    }
                }
            }
        }
    }
}

// ============================================================================
// CommandPalette Component
// ============================================================================

#[cfg(feature = "dioxus-ui")]
#[derive(Props, Clone, PartialEq)]
pub struct CommandPaletteProps {
    pub open: bool,
    pub groups: Vec<CommandGroup>,
    pub on_close: EventHandler<()>,
    pub on_select: EventHandler<String>,
    pub placeholder: Option<String>,
}

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn CommandPalette(props: CommandPaletteProps) -> Element {
    if !props.open {
        return rsx! { {} };
    }

    let mut search_input = use_signal(String::new);
    let mut selected_index = use_signal(|| 0usize);
    let _theme_state = use_context::<ThemeState>();

    let query = search_input.read().clone();
    let filtered_command_ids: Vec<String> = if query.is_empty() {
        props
            .groups
            .iter()
            .flat_map(|g| g.commands.iter().map(|c| c.id.clone()))
            .collect()
    } else {
        let query_lower = query.to_lowercase();
        props
            .groups
            .iter()
            .flat_map(|g| g.commands.iter())
            .filter(|c| {
                c.label.to_lowercase().contains(&query_lower)
                    || c.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .map(|c| c.id.clone())
            .collect()
    };

    let filtered_groups: Vec<CommandGroup> = if query.is_empty() {
        props.groups.clone()
    } else {
        let query_lower = query.to_lowercase();
        props
            .groups
            .iter()
            .filter(|g| {
                g.commands.iter().any(|c| {
                    c.label.to_lowercase().contains(&query_lower)
                        || c.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                })
            })
            .cloned()
            .collect()
    };

    let filtered_count = filtered_command_ids.len();
    let selected_id = filtered_command_ids.get(*selected_index.read()).cloned();
    let text_muted = "var(--color-text-tertiary)";
    let placeholder_text = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Search commands...".to_string());

    use_effect(move || {
        let _q = search_input.read().clone();
        selected_index.set(0);
    });

    let on_close = props.on_close;
    let on_select = props.on_select;
    let on_keydown = move |evt: KeyboardEvent| {
        let key = evt.key();
        match key {
            Key::Escape => {
                on_close.call(());
            }
            Key::ArrowDown => {
                evt.prevent_default();
                let current = *selected_index.read();
                let len = filtered_command_ids.len();
                if len > 0 {
                    selected_index.set((current + 1) % len);
                }
            }
            Key::ArrowUp => {
                evt.prevent_default();
                let current = *selected_index.read();
                let len = filtered_command_ids.len();
                if len > 0 {
                    selected_index.set(if current == 0 { len - 1 } else { current - 1 });
                }
            }
            Key::Enter => {
                let current = *selected_index.read();
                if let Some(id) = filtered_command_ids.get(current) {
                    on_select.call(id.clone());
                }
            }
            _ => {}
        }
    };

    let groups_clone = filtered_groups.clone();
    let selected_id_clone = selected_id.clone();
    let search_query_clone = search_input.read().clone();
    let on_select_clone = props.on_select;
    let on_close_clone = props.on_close;

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-start justify-center pt-[15vh]",
            onkeydown: on_keydown,
            div {
                class: "fixed inset-0 bg-black/50 backdrop-blur-sm",
                onclick: move |_| {
                    on_close_clone.call(());
                }
            }
            div {
                class: "relative w-full max-w-xl mx-4 rounded-xl shadow-2xl border overflow-hidden",
                style: "background-color: var(--color-bg-elevated); border-color: var(--color-border);",
                div {
                    class: "flex items-center px-4 border-b",
                    style: "border-color: var(--color-border);",
                    div {
                        class: "flex-shrink-0 w-5 h-5 mr-3",
                        style: "color: {text_muted};",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            }
                        }
                    }
                    input {
                        class: "flex-1 py-4 bg-transparent border-none outline-none text-base",
                        style: "color: var(--color-text-primary);",
                        r#type: "text",
                        placeholder: "{placeholder_text}",
                        value: "{search_input}",
                        oninput: move |evt| {
                            search_input.set(evt.value().to_string());
                        },
                    }
                    div {
                        class: "flex-shrink-0 ml-2",
                        style: "color: {text_muted};",
                        kbd {
                            class: "text-xs font-mono px-2 py-1 rounded bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600",
                            "esc"
                        }
                    }
                }
                div {
                    class: "max-h-96 overflow-y-auto p-2",
                    if groups_clone.is_empty() {
                        div {
                            class: "px-4 py-8 text-center",
                            style: "color: {text_muted};",
                            "No commands found"
                        }
                    } else {
                        for group in groups_clone {
                            CommandGroupSection {
                                key: "{group.name}",
                                group: group,
                                selected_id: selected_id_clone.clone(),
                                search_query: search_query_clone.clone(),
                                on_select: on_select_clone,
                            }
                        }
                    }
                }
                div {
                    class: "flex items-center justify-between px-4 py-2 border-t text-xs",
                    style: "border-color: var(--color-border);",
                    div {
                        style: "color: {text_muted};",
                        span { class: "mr-3", "navigate" }
                        span { class: "mr-3", "select" }
                        span { "close" }
                    }
                    div {
                        style: "color: {text_muted};",
                        if filtered_count > 0 {
                            span {
                                "{filtered_count} commands"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Keyboard Shortcut Hook
// ============================================================================

/// Hook to register global Cmd+K / Ctrl+K shortcut for command palette
#[cfg(feature = "dioxus-ui")]
pub fn use_command_palette_shortcut(_open_signal: Signal<bool>) {
    // Platform-specific keyboard shortcut registration would go here
}
