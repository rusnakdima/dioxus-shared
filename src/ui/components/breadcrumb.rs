//! Breadcrumb component
//!
//! Navigation breadcrumb with separator icons (chevron, slash, or dot).
//! Follows Notion-style clean, minimal design.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
use crate::schema::CanvasElement;

/// Separator style for breadcrumb items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BreadcrumbSeparator {
    #[default]
    Chevron,
    Slash,
    Dot,
}

/// Breadcrumb item with label and optional href
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

/// Render a Breadcrumb from a CanvasElement schema.
#[cfg(feature = "dioxus-ui")]
pub fn render_breadcrumb(element: &CanvasElement, _classes: &str, _is_dark: bool) -> Element {
    let items_raw = element
        .props
        .get("items")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|v| {
                    let label = v
                        .get("label")
                        .and_then(|l| l.as_str())
                        .unwrap_or("")
                        .to_string();
                    let href = v.get("href").and_then(|h| h.as_str()).map(String::from);
                    BreadcrumbItem { label, href }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let separator_str = element
        .props
        .get("separator")
        .and_then(|v| v.as_str())
        .unwrap_or("chevron");

    let separator = match separator_str {
        "slash" => BreadcrumbSeparator::Slash,
        "dot" => BreadcrumbSeparator::Dot,
        _ => BreadcrumbSeparator::Chevron,
    };

    let sep_class = "text-gray-400 dark:text-gray-500 text-base";
    let icon_name = "chevron_right";

    rsx! {
        nav {
            aria_label: "Breadcrumb",
            ol {
                class: "inline-flex items-center gap-1.5 text-sm",
                for (i, item) in items_raw.iter().enumerate() {
                    if i > 0 {
                        if separator == BreadcrumbSeparator::Chevron {
                            span {
                                class: "material-symbols-rounded {sep_class}",
                                "{icon_name}"
                            }
                        } else if separator == BreadcrumbSeparator::Slash {
                            span {
                                class: "{sep_class}",
                                "/"
                            }
                        } else {
                            span {
                                class: "{sep_class}",
                                "·"
                            }
                        }
                    }
                    li {
                        if let Some(href) = &item.href {
                            if i == items_raw.len() - 1 {
                                // Last item with href - still clickable but aria-current
                                a {
                                    href: "{href}",
                                    aria_current: "page",
                                    class: "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors",
                                    "{item.label}"
                                }
                            } else {
                                a {
                                    href: "{href}",
                                    class: "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors",
                                    "{item.label}"
                                }
                            }
                        } else {
                            // No href - this is the current page
                            span {
                                aria_current: "page",
                                class: if i == items_raw.len() - 1 {
                                    "text-gray-900 dark:text-gray-100 font-medium"
                                } else {
                                    "text-gray-500 dark:text-gray-400"
                                },
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Breadcrumb component for direct use.
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Breadcrumb(items: Vec<(String, Option<String>)>, separator: BreadcrumbSeparator) -> Element {
    let items: Vec<BreadcrumbItem> = items
        .into_iter()
        .map(|(label, href)| BreadcrumbItem { label, href })
        .collect();

    let sep_class = "text-gray-400 dark:text-gray-500 text-base";
    let icon_name = "chevron_right";

    rsx! {
        nav {
            aria_label: "Breadcrumb",
            ol {
                class: "inline-flex items-center gap-1.5 text-sm",
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        if separator == BreadcrumbSeparator::Chevron {
                            span {
                                class: "material-symbols-rounded {sep_class}",
                                "{icon_name}"
                            }
                        } else if separator == BreadcrumbSeparator::Slash {
                            span {
                                class: "{sep_class}",
                                "/"
                            }
                        } else {
                            span {
                                class: "{sep_class}",
                                "·"
                            }
                        }
                    }
                    li {
                        if let Some(href) = &item.href {
                            if i == items.len() - 1 {
                                a {
                                    href: "{href}",
                                    aria_current: "page",
                                    class: "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors",
                                    "{item.label}"
                                }
                            } else {
                                a {
                                    href: "{href}",
                                    class: "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors",
                                    "{item.label}"
                                }
                            }
                        } else {
                            span {
                                aria_current: "page",
                                class: if i == items.len() - 1 {
                                    "text-gray-900 dark:text-gray-100 font-medium"
                                } else {
                                    "text-gray-500 dark:text-gray-400"
                                },
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
