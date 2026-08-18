//! UI Components
//!
//! Provides schema-driven UI components for Dioxus applications.

pub mod accordion;
pub mod action_bus;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod command_palette;
pub mod container_components;
pub mod dropdown;
pub mod dynamic_page;
pub mod dynamic_renderer;
pub mod form_components;
pub mod input;
pub mod interactive_components;
pub mod modal;
pub mod pagination;
pub mod progress;
pub mod skeleton;
pub mod table;
pub mod theme_provider;

pub use crate::themes::ThemeMode;
pub use accordion::{
    Accordion, AccordionConfig, AccordionItem, AccordionItemConfig, AccordionVariant,
};
pub use action_bus::{ActionBus, AppAction, NavigateAction};
pub use badge::{Badge, BadgeVariant};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbSeparator};
pub use button::{Button, ButtonConfig, ButtonVariant};
pub use card::{Card, CardConfig, CardVariant};
pub use command_palette::{
    use_command_palette_shortcut, CommandGroup, CommandItem, CommandPalette,
};
pub use dropdown::{
    Dropdown, DropdownDivider, DropdownItem, DropdownItems, DropdownMenuItemProps, DropdownPosition,
};
pub use dynamic_page::DynamicPage;
pub use dynamic_renderer::DynamicRenderer;
pub use input::{Input, Select, SelectOption, Textarea};
pub use modal::{render_dialog, Dialog, DialogVariant, Modal, ModalSize};
pub use pagination::Pagination;
pub use progress::{Progress, ProgressSize, ProgressVariant};
pub use skeleton::{SkeletonCircle, SkeletonRect, SkeletonText};
pub use table::{
    PaginationControls, SortState, Table, TableCell, TableColumnHeader, TableHeader, TableRow,
    TableSkeletonRow, TableVariant,
};
pub use theme_provider::{
    use_theme_mode, use_theme_variant, use_toggle_theme, ThemeProvider, ThemeState, ThemeToggle,
};
