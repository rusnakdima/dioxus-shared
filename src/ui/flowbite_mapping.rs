//! Flowbite class resolution — re-export barrel
//!
//! This module re-exports all public APIs from the split modules for
//! backward compatibility. New code should import directly from:
//! - `render_component::resolve_flowbite_classes`
//! - `layout_engine::resolve_layout_classes`

pub use super::layout_engine::resolve_layout_classes;
pub use super::render_component::resolve_flowbite_classes;
