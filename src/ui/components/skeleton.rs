//! Skeleton component
//!
//! Loading placeholder states with animated pulse effect

#[cfg(feature = "dioxus-ui")]
use crate::themes::ThemeMode;
#[cfg(feature = "dioxus-ui")]
use crate::ui::components::theme_provider::ThemeState;
#[cfg(feature = "dioxus-ui")]
use dioxus::prelude::*;

/// Skeleton color scheme for light/dark mode
#[cfg(feature = "dioxus-ui")]
fn skeleton_style(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::Light | ThemeMode::System => "background-color: #e5e7eb;",
        ThemeMode::Dark => "background-color: #374151;",
    }
}

/// Skeleton pulse animation CSS
#[cfg(feature = "dioxus-ui")]
const SKELETON_ANIMATION: &str = r#"
@keyframes skeleton-pulse {
    0% { opacity: 1; }
    50% { opacity: 0.4; }
    100% { opacity: 1; }
}
.skeleton-pulse {
    animation: skeleton-pulse 1.5s ease-in-out infinite;
}
"#;

/// SkeletonText - single line text placeholder
///
/// # Arguments
/// * `width` - Width of the skeleton line (e.g., "100%", "200px", "12rem")
/// * `height` - Height of the skeleton line (e.g., "1rem", "16px")
/// * `class` - Additional CSS classes
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn SkeletonText(width: String, height: String, class: String) -> Element {
    let theme_state = use_context::<ThemeState>();
    let mode = *theme_state.mode.read();
    let bg_style = skeleton_style(mode);
    let combined_style = format!(
        "width: {}; height: {}; border-radius: 0.25rem; {} animation: skeleton-pulse 1.5s ease-in-out infinite;",
        width, height, bg_style
    );
    let classes = format!("skeleton-pulse {}", class).trim().to_string();

    rsx! {
        style { {SKELETON_ANIMATION} }
        div {
            class: "{classes}",
            style: "{combined_style}"
        }
    }
}

/// SkeletonCircle - circular placeholder for avatar loading
///
/// # Arguments
/// * `size` - Size of the circle (width and height)
/// * `class` - Additional CSS classes
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn SkeletonCircle(size: String, class: String) -> Element {
    let theme_state = use_context::<ThemeState>();
    let mode = *theme_state.mode.read();
    let bg_style = skeleton_style(mode);
    let combined_style = format!(
        "width: {}; height: {}; border-radius: 50%; {} animation: skeleton-pulse 1.5s ease-in-out infinite;",
        size, size, bg_style
    );
    let classes = format!("skeleton-pulse {}", class).trim().to_string();

    rsx! {
        style { {SKELETON_ANIMATION} }
        div {
            class: "{classes}",
            style: "{combined_style}"
        }
    }
}

/// SkeletonRect - rectangular placeholder for card/image loading
///
/// # Arguments
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// * `border_radius` - Border radius (e.g., "0.5rem", "4px")
/// * `class` - Additional CSS classes
#[cfg(feature = "dioxus-ui")]
#[component]
pub fn SkeletonRect(
    width: String,
    height: String,
    border_radius: String,
    class: String,
) -> Element {
    let theme_state = use_context::<ThemeState>();
    let mode = *theme_state.mode.read();
    let bg_style = skeleton_style(mode);
    let combined_style = format!(
        "width: {}; height: {}; border-radius: {}; {} animation: skeleton-pulse 1.5s ease-in-out infinite;",
        width, height, border_radius, bg_style
    );
    let classes = format!("skeleton-pulse {}", class).trim().to_string();

    rsx! {
        style { {SKELETON_ANIMATION} }
        div {
            class: "{classes}",
            style: "{combined_style}"
        }
    }
}
