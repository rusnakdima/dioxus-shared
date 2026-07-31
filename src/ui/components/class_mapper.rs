//! ClassMapper - maps semantic class names to theme-aware Tailwind classes
//!
//! Takes schema classes with semantic names and translates them to theme-aware
//! classes using CSS variables. E.g.:
//!   "bg-surface text-primary" → "bg-[var(--color-bg-surface)] text-[var(--color-text-primary)]"
//!
//! Uses ThemeVariant to determine the appropriate CSS variable values at render time.

#[cfg(feature = "dioxus-ui")]
use crate::themes::ThemeVariant;

#[cfg(feature = "dioxus-ui")]
use std::collections::HashMap;

#[cfg(feature = "dioxus-ui")]
pub struct ClassMapper;

#[cfg(feature = "dioxus-ui")]
impl ClassMapper {
    fn semantic_map(_is_dark: bool) -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        
        map.insert("bg-surface", "bg-[var(--color-bg-surface)]");
        map.insert("bg-elevated", "bg-[var(--color-bg-elevated)]");
        map.insert("bg-backdrop", "bg-[var(--color-bg-backdrop)]");
        map.insert("bg-primary", "bg-[var(--color-bg-primary)]");
        map.insert("bg-secondary", "bg-[var(--color-bg-secondary)]");
        
        map.insert("text-primary", "text-[var(--color-text-primary)]");
        map.insert("text-secondary", "text-[var(--color-text-secondary)]");
        map.insert("text-muted", "text-[var(--color-text-muted)]");
        map.insert("text-inverse", "text-[var(--color-text-inverse)]");
        
        map.insert("accent-primary", "text-[var(--color-accent-primary)]");
        map.insert("accent-hover", "text-[var(--color-accent-hover)]");
        
        map.insert("border-default", "border-[var(--color-border)]");
        map.insert("border-subtle", "border-[var(--color-border-subtle)]");
        map.insert("border-focus", "ring-[var(--color-border-focus)]");
        
        map.insert("radius", "rounded-[var(--radius)]");
        map.insert("shadow", "shadow-[var(--shadow)]");
        
        map.insert("success", "text-[var(--color-success)]");
        map.insert("warning", "text-[var(--color-warning)]");
        map.insert("error", "text-[var(--color-error)]");
        
        map
    }

    pub fn map_single(class: &str, _variant: ThemeVariant, _is_dark: bool) -> String {
        let semantic_map = Self::semantic_map(_is_dark);
        semantic_map.get(class).unwrap_or(&class).to_string()
    }

    pub fn map_all(classes: &str, variant: ThemeVariant, is_dark: bool) -> String {
        if classes.is_empty() {
            return String::new();
        }
        
        classes
            .split_whitespace()
            .map(|cls| Self::map_single(cls, variant, is_dark))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn merge_with_defaults(classes: &str, variant: ThemeVariant, is_dark: bool) -> String {
        let _semantic_map = Self::semantic_map(is_dark);

        let defaults = vec![
            "bg-[var(--color-bg-surface)]",
            "text-[var(--color-text-primary)]",
            "border-[var(--color-border-subtle)]",
            "rounded-[var(--radius)]",
        ];
        
        let mut class_map = HashMap::new();
        
        for cls in defaults.iter() {
            let key = cls.split(|c| c == '-' || c == '[' || c == '(' || c == ')').next().unwrap_or(cls);
            class_map.insert(key.to_string(), (*cls).to_string());
        }
        
        for cls in classes.split_whitespace() {
            let mapped = Self::map_single(cls, variant, is_dark);
            let key = mapped.split(|c| c == '-' || c == '[' || c == '(' || c == ')').next().unwrap_or(&mapped);
            class_map.insert(key.to_string(), mapped);
        }
        
        class_map.into_values().collect::<Vec<_>>().join(" ")
    }
}
