//! UI Components
//!
//! Provides schema-driven UI components for Dioxus applications.

pub mod button;
pub mod input;
pub mod card;
pub mod modal;
pub mod badge;
pub mod dynamic_page;
pub mod dynamic_renderer;
pub mod theme_provider;
pub mod action_bus;
pub mod class_mapper;

pub use button::{Button, ButtonVariant, ButtonConfig};
pub use input::{Input, Textarea, Select, SelectOption};
pub use card::{Card, CardVariant, CardConfig};
pub use modal::{Modal, Dialog, ModalSize};
pub use badge::{Badge, BadgeVariant};
pub use dynamic_page::DynamicPage;
pub use dynamic_renderer::DynamicRenderer;
pub use theme_provider::{ThemeProvider, use_theme_mode, use_toggle_theme, use_theme_variant, ThemeToggle, ThemeState};
pub use action_bus::{ActionBus, AppAction, NavigateAction};
pub use class_mapper::ClassMapper;
pub use crate::themes::ThemeMode;
