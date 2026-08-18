#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Pagination(
    current_page: usize,
    total_pages: usize,
    visible_window: usize,
    on_page_change: Option<Callback<usize>>,
) -> Element {
    let prev_disabled = current_page <= 1;
    let next_disabled = current_page >= total_pages;

    let page_items: Vec<(usize, bool)> = pages_to_items(current_page, total_pages, visible_window);

    rsx! {
        nav {
            class: "flex items-center gap-1",
            role: "navigation",
            aria_label: "Pagination",

            button {
                class: "px-3 py-2 text-sm rounded-lg border transition-colors",
                style: if prev_disabled {
                    "color: var(--color-text-placeholder); cursor: not-allowed; opacity: 0.5; border-color: var(--color-border);"
                } else {
                    "color: var(--color-text-secondary); border-color: var(--color-border);"
                },
                disabled: prev_disabled,
                aria_disabled: prev_disabled,
                aria_label: "Previous page",
                onclick: move |_| {
                    if let Some(cb) = &on_page_change {
                        cb.call(current_page.saturating_sub(1));
                    }
                },
                "Previous"
            }

            for (page_num, is_current) in page_items {
                if page_num == 0 {
                    span {
                        class: "px-2 py-2 text-sm",
                        style: "color: var(--color-text-placeholder);",
                        "..."
                    }
                } else {
                    button {
                        class: "min-w-[40px] px-3 py-2 text-sm rounded-lg border transition-colors",
                        style: if is_current {
                            "color: var(--color-text-inverse); background-color: var(--color-accent); border-color: var(--color-accent); font-weight: 500;"
                        } else {
                            "color: var(--color-text-secondary); border-color: var(--color-border);"
                        },
                        aria_current: if is_current { "page" } else { None },
                        onclick: move |_evt| {
                            if let Some(cb) = &on_page_change {
                                cb.call(page_num);
                            }
                        },
                        "{page_num}"
                    }
                }
            }

            button {
                class: "px-3 py-2 text-sm rounded-lg border transition-colors",
                style: if next_disabled {
                    "color: var(--color-text-placeholder); cursor: not-allowed; opacity: 0.5; border-color: var(--color-border);"
                } else {
                    "color: var(--color-text-secondary); border-color: var(--color-border);"
                },
                disabled: next_disabled,
                aria_disabled: next_disabled,
                aria_label: "Next page",
                onclick: move |_| {
                    if let Some(cb) = &on_page_change {
                        cb.call(current_page.saturating_add(1).min(total_pages));
                    }
                },
                "Next"
            }
        }
    }
}

fn pages_to_items(current: usize, total: usize, window: usize) -> Vec<(usize, bool)> {
    if total <= 1 {
        return vec![];
    }

    let half_window = window / 2;
    let mut pages: Vec<usize> = Vec::new();

    pages.push(1);

    let start = current.saturating_sub(half_window);
    let end = (current + half_window).min(total);

    if start > 2 {
        pages.push(0);
    }

    for page in start.max(2)..=end {
        if page > 1 && page < total {
            pages.push(page);
        }
    }

    if end < total - 1 {
        pages.push(0);
    }

    if total > 1 {
        pages.push(total);
    }

    pages.sort();
    pages.dedup();

    let mut result: Vec<(usize, bool)> = Vec::new();
    let mut iter = pages.iter().peekable();
    while let Some(&page) = iter.next() {
        if page == 0 {
            let has_prev = result.last().map(|(p, _)| *p != 0).unwrap_or(false);
            let has_next = iter.peek().map(|&&p| p != 0).unwrap_or(false);
            if has_prev && has_next {
                result.push((0, false));
            }
        } else {
            result.push((page, page == current));
        }
    }

    result
}
