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

pub use components::*;
