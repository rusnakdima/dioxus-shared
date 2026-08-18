//! UI components for Dioxus applications
//!
//! This module provides schema-driven UI components that work with the SignalStore
//! and SDUI schema system. Components are styled with TailwindCSS and support
//! Light/Dark themes.
//!
//! # Example
//!
//! ```rust,no_run
//! use dioxus_shared::ui::components::{Button, Card, DynamicPage};
//! use dioxus_shared::ui::ThemeProvider;
//! ```

pub mod components;
pub mod css_var_utils;
pub mod flowbite;
pub mod flowbite_mapping;
pub mod layout_engine;
pub mod render_component;

pub use components::*;
