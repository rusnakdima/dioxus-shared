//! Theme Provider component
//!
//! Provides theme context for Light/Dark/System mode switching with 7-variant support.
//! This is the single library-level owner of the theme signal so every page-level component
//! inherits the same dark/light/variant state without app-level duplication.

#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus-ui")]
use crate::themes::{generate_css_vars, ThemeMode, ThemeVariant};

#[cfg(feature = "dioxus-ui")]
#[derive(Clone, Copy)]
pub struct ThemeState {
    pub mode: Signal<ThemeMode>,
    pub variant: Signal<ThemeVariant>,
}

#[cfg(feature = "dioxus-ui")]
impl ThemeState {
    pub fn is_dark(&self) -> bool {
        matches!(*self.mode.read(), ThemeMode::Dark)
    }
}

#[cfg(feature = "dioxus-ui")]
fn toggle_mode_in_place(mut mode: Signal<ThemeMode>) {
    let current = *mode.read();
    let next = match current {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
        ThemeMode::System => ThemeMode::Light,
    };
    mode.set(next);
}

#[cfg(feature = "dioxus-ui")]
fn variant_css_class(variant: ThemeVariant) -> &'static str {
    match variant {
        ThemeVariant::MaterialDesign3 => "material-design-v3",
        ThemeVariant::Glassmorphism => "glassmorphism",
        ThemeVariant::Claymorphism => "claymorphism",
        ThemeVariant::Skeuomorphism => "skeuomorphism",
        ThemeVariant::NeoBrutalism => "neo-brutalism",
        ThemeVariant::Brutalism => "brutalism",
        ThemeVariant::Neumorphism => "neumorphism",
    }
}

#[cfg(feature = "dioxus-ui")]
fn mode_attr(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::Dark => "dark",
        ThemeMode::Light => "light",
        ThemeMode::System => "system",
    }
}

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn ThemeProvider(
    initial_mode: ThemeMode,
    initial_variant: ThemeVariant,
    children: Element,
) -> Element {
    let mode = use_signal(|| initial_mode);
    let variant = use_signal(|| initial_variant);
    let is_dark = use_memo(move || matches!(*mode.read(), ThemeMode::Dark));

    provide_context(ThemeState { mode, variant });

    let css_vars = use_memo(move || {
        generate_css_vars(*variant.read(), *mode.read())
    });

    rsx! {
        div {
            class: if *is_dark.read() { "dark" } else { "" },
            "data-theme": "{variant_css_class(*variant.read())}",
            "data-theme-mode": "{mode_attr(*mode.read())}",
            style { "{css_vars.read()}" }
            {children}
        }
    }
}

#[cfg(feature = "dioxus-ui")]
pub fn use_theme_mode() -> ThemeMode {
    let ctx = use_context::<ThemeState>();
    let mode = *ctx.mode.read();
    mode
}

#[cfg(feature = "dioxus-ui")]
pub fn use_theme_variant() -> ThemeVariant {
    let ctx = use_context::<ThemeState>();
    let variant = *ctx.variant.read();
    variant
}

#[cfg(feature = "dioxus-ui")]
pub fn use_toggle_theme() -> impl Fn() {
    let ctx = use_context::<ThemeState>();
    move || toggle_mode_in_place(ctx.mode)
}

#[cfg(feature = "dioxus-ui")]
pub fn use_set_variant() -> impl FnMut(ThemeVariant) + 'static {
    let ctx = use_context::<ThemeState>();
    let mut variant = ctx.variant;
    move |new_variant| variant.set(new_variant)
}

#[cfg(feature = "dioxus-ui")]
pub fn resolve_is_dark(mode: ThemeMode) -> bool {
    matches!(mode, ThemeMode::Dark)
}

#[cfg(feature = "dioxus-ui")]
#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            class: "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-sm p-2",
            r#type: "button",
            title: "Toggle theme",
            onclick: move |_| {
                let ctx = use_context::<ThemeState>();
                toggle_mode_in_place(ctx.mode);
            },

            svg {
                class: "w-5 h-5",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path {
                    d: "M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"
                }
            }
        }
    }
}
