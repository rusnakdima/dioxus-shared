//! Table component
//!
//! Provides table components with sorting, pagination, and variants

#[cfg(feature = "dioxus-ui")]
use crate::themes::tokens::flowbite_classes::table as table_classes;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Sort state for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortState {
    #[default]
    None,
    Asc,
    Desc,
}

/// Table style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableVariant {
    #[default]
    Default,
    Striped,
    Bordered,
}

/// Pagination info
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationInfo {
    pub current_page: usize,
    pub total_pages: usize,
    pub page_size: usize,
    pub total_items: usize,
}

impl Default for PaginationInfo {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            page_size: 10,
            total_items: 0,
        }
    }
}

#[cfg(feature = "dioxus-ui")]
impl TableVariant {
    pub fn as_classes(&self) -> &'static str {
        match self {
            TableVariant::Default => "",
            TableVariant::Striped => table_classes::STRIPED,
            TableVariant::Bordered => table_classes::BORDERED,
        }
    }
}

/// Table component - main container
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Table(
    /// Additional CSS classes
    class: String,
    /// Table variant (striped, bordered)
    variant: TableVariant,
    /// Show loading skeleton state
    loading: bool,
    /// Pagination info
    pagination: Option<PaginationInfo>,
    /// Callback for page change, receives new page number
    on_page_change: Option<Callback<usize>>,
    children: Element,
) -> Element {
    let variant_classes = variant.as_classes();
    let classes = format!("{} {}", table_classes::DEFAULT, variant_classes)
        .trim()
        .to_string();

    rsx! {
        div {
            class: "overflow-x-auto {classes} {class}",
            style: "background-color: var(--color-bg-surface); border-radius: var(--color-radius);",

            table {
                class: "w-full text-sm text-left",
                style: "color: var(--color-text-primary);",

                {children}
            }

            if let Some(pag) = pagination {
                PaginationControls {
                    pagination: pag,
                    on_page_change: on_page_change,
                }
            }
        }
    }
}

/// Table header row
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn TableHeader(children: Element) -> Element {
    rsx! {
        thead {
            class: "text-xs uppercase",
            style: "color: var(--color-text-secondary); background-color: var(--color-bg-elevated);",
            tr { {children} }
        }
    }
}

/// Column header with optional sorting
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn TableColumnHeader(
    /// Column label
    label: String,
    /// Enable sorting for this column
    sortable: bool,
    /// Current sort state
    sort_state: SortState,
    /// Callback when sort is clicked
    on_sort: Option<Callback<()>>,
) -> Element {
    let sort_icon = match sort_state {
        SortState::None => "unfold_more",
        SortState::Asc => "expand_less",
        SortState::Desc => "expand_more",
    };

    let cursor_class = if sortable {
        "cursor-pointer select-none"
    } else {
        ""
    };

    rsx! {
        th {
            class: "px-4 py-3 {cursor_class}",
            style: "border-bottom: 1px solid var(--color-border);",

            if sortable {
                div {
                    class: "flex items-center gap-1",
                    onclick: move |_| {
                        if let Some(cb) = on_sort {
                            cb.call(());
                        }
                    },
                    "{label}"
                    span {
                        class: "material-symbols-rounded text-base",
                        style: "color: var(--color-text-secondary);",
                        "{sort_icon}"
                    }
                }
            } else {
                "{label}"
            }
        }
    }
}

/// Table body row
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn TableRow(
    /// Row index for striped styling
    index: usize,
    children: Element,
) -> Element {
    let striped_class = if index % 2 == 1 { "bg-gray-50" } else { "" };

    rsx! {
        tr {
            class: "border-b {striped_class}",
            style: "border-color: var(--color-border);",
            {children}
        }
    }
}

/// Table cell (td)
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn TableCell(
    /// Cell content
    children: Element,
) -> Element {
    rsx! {
        td {
            class: "px-4 py-3",
            style: "border-bottom: 1px solid var(--color-border);",
            {children}
        }
    }
}

/// Loading skeleton row
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn TableSkeletonRow(
    /// Number of columns
    columns: usize,
) -> Element {
    rsx! {
        tr {
            style: "border-color: var(--color-border);",

            for _ in 0..columns {
                td {
                    class: "px-4 py-3",
                    style: "border-bottom: 1px solid var(--color-border);",
                    div {
                        class: "animate-pulse h-4 bg-gray-200 rounded dark:bg-gray-700",
                        style: "width: 75%;",
                    }
                }
            }
        }
    }
}

/// Pagination controls
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn PaginationControls(
    pagination: PaginationInfo,
    on_page_change: Option<Callback<usize>>,
) -> Element {
    let prev_disabled = pagination.current_page <= 1;
    let next_disabled = pagination.current_page >= pagination.total_pages;

    let start_item = ((pagination.current_page - 1) * pagination.page_size) + 1;
    let end_item = (pagination.current_page * pagination.page_size).min(pagination.total_items);

    rsx! {
        div {
            class: "flex items-center justify-between px-4 py-3",
            style: "border-top: 1px solid var(--color-border);",

            div {
                class: "text-sm text-gray-500 dark:text-gray-400",
                style: "color: var(--color-text-secondary);",
                "Showing {start_item} to {end_item} of {pagination.total_items} entries"
            }

            div {
                class: "flex gap-1",
                button {
                    class: "px-3 py-1 text-sm rounded border",
                    style: if prev_disabled {
                        "color: var(--color-text-placeholder); cursor: not-allowed; opacity: 0.5;"
                    } else {
                        "color: var(--color-text-primary); border-color: var(--color-border);"
                    },
                    disabled: prev_disabled,
                    onclick: move |_| {
                        if let Some(cb) = &on_page_change {
                            cb.call(pagination.current_page - 1);
                        }
                    },
                    "Previous"
                }

                button {
                    class: "px-3 py-1 text-sm rounded border",
                    style: if next_disabled {
                        "color: var(--color-text-placeholder); cursor: not-allowed; opacity: 0.5;"
                    } else {
                        "color: var(--color-text-primary); border-color: var(--color-border);"
                    },
                    disabled: next_disabled,
                    onclick: move |_| {
                        if let Some(cb) = &on_page_change {
                            cb.call(pagination.current_page + 1);
                        }
                    },
                    "Next"
                }
            }
        }
    }
}
