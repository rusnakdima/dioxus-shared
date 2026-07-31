//! Modal and Dialog components
//!
//! Provides modal dialogs with overlay

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Modal sizes
#[cfg(feature = "dioxus-ui")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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

/// Modal component
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn Modal(
    open: bool,
    title: String,
    size: ModalSize,
    class: String,
    on_close: EventHandler<()>,
    dark_mode: bool,
    children: Element,
) -> Element {
    if !open {
        return rsx! { {} };
    }

    let size_classes = size.as_classes();
    let bg_class = if dark_mode { "dark:bg-gray-800" } else { "bg-white" };
    let border_class = if dark_mode { "dark:border-gray-700" } else { "border-gray-200" };
    let header_text_class = if dark_mode { "dark:text-white" } else { "text-gray-900" };
    let close_btn_class = if dark_mode { "dark:hover:text-gray-300" } else { "hover:text-gray-600" };
    let classes = format!("{} {} {}", size_classes, bg_class, class).trim().to_string();

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
            onclick: move |_| { on_close.call(()); },

            div {
                class: "{classes} w-full rounded-lg shadow-xl",
                onclick: move |evt| { evt.stop_propagation(); },

                div {
                    class: "flex items-center justify-between p-4 border-b {border_class}",

                    h3 {
                        class: "text-lg font-medium {header_text_class}",
                        "{title}"
                    }

                    button {
                        class: "text-gray-400 {close_btn_class}",
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
                    class: "flex items-center justify-end p-4 border-t {border_class} space-x-2",
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
    
    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50",
            onclick: move |_| { on_cancel.call(()); },
            
            // Dialog content
            div {
                class: "max-w-sm w-full bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 {class}",
                onclick: move |evt| { evt.stop_propagation(); },
                
                // Message
                p {
                    class: "mb-6 text-gray-700 dark:text-gray-300",
                    "{message}"
                }
                
                // Buttons
                div {
                    class: "flex justify-end space-x-3",
                    
                    button {
                        class: "px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 dark:text-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600",
                        r#type: "button",
                        onclick: move |_| { on_cancel.call(()); },
                        
                        "{cancel_label}"
                    }
                    
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700",
                        r#type: "button",
                        onclick: move |_| { on_confirm.call(()); },
                        
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
