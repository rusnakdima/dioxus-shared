//! Flowbite theme integration for SDUI.
//!
//! This module provides Flowbite-styled Tailwind classes driven by the `variant` field
//! on CanvasElement. It replaces the old hardcoded class strings in dynamic_renderer.rs.
//!
//! Flowbite CDN is loaded once on startup via `init_flowbite_css()`.
//! The actual class resolution happens in `flowbite_mapping.rs`.

#[cfg(feature = "dioxus-ui")]
use crate::themes::ThemeMode;

/// Flowbite color tokens for light mode variants.
/// These map UniversalVariant (primary/secondary/outline/...) to CSS color values.
#[derive(Debug, Clone)]
pub struct FlowbiteTheme {
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub accent: String,
}

impl FlowbiteTheme {
    /// Returns the Flowbite theme for light mode
    pub fn light() -> Self {
        Self {
            primary: "#4F46E5".into(),   // indigo-600
            secondary: "#6B7280".into(), // gray-500
            success: "#10B981".into(),   // emerald-500
            warning: "#F59E0B".into(),   // amber-500
            danger: "#EF4444".into(),    // red-500
            info: "#3B82F6".into(),      // blue-500
            accent: "#6366F1".into(),    // indigo-500
        }
    }

    /// Returns the Flowbite theme for dark mode
    pub fn dark() -> Self {
        Self {
            primary: "#818CF8".into(),   // indigo-400
            secondary: "#9CA3AF".into(), // gray-400
            success: "#34D399".into(),   // emerald-400
            warning: "#FBBF24".into(),   // amber-400
            danger: "#F87171".into(),    // red-400
            info: "#60A5FA".into(),      // blue-400
            accent: "#A5B4FC".into(),    // indigo-400
        }
    }

    /// Returns theme based on mode
    pub fn for_mode(mode: ThemeMode, system_is_dark: bool) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
            ThemeMode::System => {
                if system_is_dark {
                    Self::dark()
                } else {
                    Self::light()
                }
            }
        }
    }
}

#[cfg(all(feature = "dioxus-ui", target_arch = "wasm32"))]
#[allow(dead_code)]
static FLOWBITE_INIT: std::sync::Once = std::sync::Once::new();

#[cfg(all(feature = "dioxus-ui", target_arch = "wasm32"))]
#[allow(dead_code)]
static FLOWBITE_LOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Initialize Flowbite CSS by injecting the CDN link + custom theme CSS into the document head.
/// Safe to call multiple times — only runs once.
#[cfg(all(feature = "dioxus-ui", target_arch = "wasm32"))]
pub fn init_flowbite_css() {
    use web_sys::wasm_bindgen::JsCast;

    FLOWBITE_INIT.call_once(|| {
        // Check if already loaded
        if FLOWBITE_LOADED.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };
        let head = match document.head() {
            Some(h) => h,
            None => return,
        };

        // Inject Flowbite CDN CSS
        let flowbite_link = document
            .create_element("link")
            .unwrap()
            .dyn_into::<web_sys::Element>()
            .unwrap();
        let _ = flowbite_link.set_attribute("rel", "stylesheet");
        let _ = flowbite_link.set_attribute(
            "href",
            "https://cdn.jsdelivr.net/npm/flowbite@2.5.2/dist/flowbite.min.css",
        );
        let _ = flowbite_link.set_attribute("type", "text/css");
        let _ = head.append_child(&flowbite_link);

        // Mark as loaded
        FLOWBITE_LOADED.store(true, std::sync::atomic::Ordering::SeqCst);
    });
}
