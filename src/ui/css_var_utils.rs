//! CSS variable helper utilities for inline styles.
//!
//! Provides helper functions to generate inline CSS style strings for CSS variable
//! references, replacing the invalid Tailwind arbitrary value patterns like
//! `bg-[var(--color-accent-primary)]` that Tailwind JIT cannot resolve.

/// Creates a `background-color` inline style from a CSS variable reference.
///
/// # Arguments
///
/// * `var_name` - A CSS variable reference string (e.g., `var(--color-primary)`)
///
/// # Returns
///
/// A formatted string: `background-color: <var_name>;`
///
/// # Example
///
/// ```
/// use dioxus_shared::ui::css_var_utils::bg_color;
///
/// let style = bg_color("var(--color-accent)");
/// assert_eq!(style, "background-color: var(--color-accent);");
/// ```
#[inline]
pub fn bg_color(var_name: &str) -> String {
    format!("background-color: {};", var_name)
}

/// Creates a `color` inline style from a CSS variable reference.
///
/// # Arguments
///
/// * `var_name` - A CSS variable reference string (e.g., `var(--text-primary)`)
///
/// # Returns
///
/// A formatted string: `color: <var_name>;`
///
/// # Example
///
/// ```
/// use dioxus_shared::ui::css_var_utils::text_color;
///
/// let style = text_color("var(--text-primary)");
/// assert_eq!(style, "color: var(--text-primary);");
/// ```
#[inline]
pub fn text_color(var_name: &str) -> String {
    format!("color: {};", var_name)
}

/// Creates a `border-color` inline style from a CSS variable reference.
///
/// # Arguments
///
/// * `var_name` - A CSS variable reference string (e.g., `var(--border-primary)`)
///
/// # Returns
///
/// A formatted string: `border-color: <var_name>;`
///
/// # Example
///
/// ```
/// use dioxus_shared::ui::css_var_utils::border_color;
///
/// let style = border_color("var(--border-primary)");
/// assert_eq!(style, "border-color: var(--border-primary);");
/// ```
#[inline]
pub fn border_color(var_name: &str) -> String {
    format!("border-color: {};", var_name)
}

/// Creates a `fill` inline style from a CSS variable reference.
///
/// # Arguments
///
/// * `var_name` - A CSS variable reference string (e.g., `var(--icon-fill)`)
///
/// # Returns
///
/// A formatted string: `fill: <var_name>;`
///
/// # Example
///
/// ```
/// use dioxus_shared::ui::css_var_utils::fill_color;
///
/// let style = fill_color("var(--icon-fill)");
/// assert_eq!(style, "fill: var(--icon-fill);");
/// ```
#[inline]
pub fn fill_color(var_name: &str) -> String {
    format!("fill: {};", var_name)
}

/// Creates a `stroke` inline style from a CSS variable reference.
///
/// # Arguments
///
/// * `var_name` - A CSS variable reference string (e.g., `var(--stroke-primary)`)
///
/// # Returns
///
/// A formatted string: `stroke: <var_name>;`
///
/// # Example
///
/// ```
/// use dioxus_shared::ui::css_var_utils::stroke_color;
///
/// let style = stroke_color("var(--stroke-primary)");
/// assert_eq!(style, "stroke: var(--stroke-primary);");
/// ```
#[inline]
pub fn stroke_color(var_name: &str) -> String {
    format!("stroke: {};", var_name)
}
