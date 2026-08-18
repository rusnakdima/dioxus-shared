//! Theme system for Dioxus applications
//!
//! Provides 12-variant theme system with abstract CSS tokens.
//! All components use `--color-*` abstract tokens for cross-variant compatibility.

pub mod tokens;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export)]
pub enum ThemeVariant {
    #[default]
    MaterialDesign3,
    Glassmorphism,
    Claymorphism,
    Skeuomorphism,
    NeoBrutalism,
    Brutalism,
    Neumorphism,
    Nord,
    TokyoNight,
    Catppuccin,
    RosePine,
    Linear,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::MaterialDesign3 => "material-design-v3",
            ThemeVariant::Glassmorphism => "glassmorphism",
            ThemeVariant::Claymorphism => "claymorphism",
            ThemeVariant::Skeuomorphism => "skeuomorphism",
            ThemeVariant::NeoBrutalism => "neo-brutalism",
            ThemeVariant::Brutalism => "brutalism",
            ThemeVariant::Neumorphism => "neumorphism",
            ThemeVariant::Nord => "nord",
            ThemeVariant::TokyoNight => "tokyo-night",
            ThemeVariant::Catppuccin => "catppuccin",
            ThemeVariant::RosePine => "rose-pine",
            ThemeVariant::Linear => "linear",
        }
    }

    /// Returns a CSS variable string for inline style use.
    /// Do NOT use in Tailwind class strings — use in style attributes.
    fn css_var(name: &str) -> String {
        format!("var(--{})", name)
    }

    pub fn bg_surface(&self) -> String {
        Self::css_var("color-bg-surface")
    }

    pub fn bg_elevated(&self) -> String {
        Self::css_var("color-bg-elevated")
    }

    pub fn bg_backdrop(&self) -> String {
        Self::css_var("color-bg-backdrop")
    }

    pub fn text_primary(&self) -> String {
        Self::css_var("color-text-primary")
    }

    pub fn text_secondary(&self) -> String {
        Self::css_var("color-text-secondary")
    }

    pub fn text_muted(&self) -> String {
        Self::css_var("color-text-muted")
    }

    pub fn accent_primary(&self) -> String {
        Self::css_var("color-accent-primary")
    }

    pub fn accent_hover(&self) -> String {
        Self::css_var("color-accent-hover")
    }

    pub fn accent_subtle(&self) -> String {
        Self::css_var("color-accent-subtle")
    }

    pub fn text_inverse(&self) -> String {
        Self::css_var("color-text-inverse")
    }

    pub fn border_default(&self) -> String {
        Self::css_var("color-border")
    }

    pub fn border_subtle(&self) -> String {
        Self::css_var("color-border-subtle")
    }

    pub fn border_focus(&self) -> String {
        Self::css_var("color-border-focus")
    }

    pub fn radius(&self) -> String {
        Self::css_var("radius")
    }

    pub fn shadow(&self) -> String {
        Self::css_var("shadow")
    }

    pub fn shadow_elevation(&self) -> String {
        Self::css_var("shadow-elevation")
    }

    pub fn success(&self) -> String {
        Self::css_var("color-success")
    }

    pub fn warning(&self) -> String {
        Self::css_var("color-warning")
    }

    pub fn error(&self) -> String {
        Self::css_var("color-error")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractTokens {
    pub bg_primary: String,
    pub bg_secondary: String,
    pub bg_tertiary: String,
    pub bg_hover: String,
    pub bg_active: String,
    pub bg_disabled: String,
    pub bg_surface: String,
    pub bg_elevated: String,
    pub bg_backdrop: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub text_disabled: String,
    pub text_inverse: String,
    pub accent_primary: String,
    pub accent_secondary: String,
    pub accent_hover: String,
    pub accent_subtle: String,
    pub border: String,
    pub border_subtle: String,
    pub border_focus: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub shadow: String,
    pub shadow_elevation: String,
    pub text_placeholder: String,
    pub radius: String,
}

impl ThemeVariant {
    pub fn light_tokens(&self) -> AbstractTokens {
        match self {
            ThemeVariant::MaterialDesign3 => AbstractTokens {
                bg_primary: "#fefbff".into(),
                bg_secondary: "#f7f2fa".into(),
                bg_tertiary: "#e8e0ec".into(),
                bg_hover: "#e7e0ec".into(),
                bg_active: "#e8e0ec".into(),
                bg_disabled: "rgba(28,27,31,0.12)".into(),
                bg_surface: "#fefbff".into(),
                bg_elevated: "#ffffff".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#1c1b1f".into(),
                text_secondary: "#49454f".into(),
                text_muted: "#79747e".into(),
                text_disabled: "rgba(28,27,31,0.38)".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#6750a4".into(),
                accent_secondary: "#625b71".into(),
                accent_hover: "#7c67ab".into(),
                accent_subtle: "#e8def8".into(),
                border: "#79747e".into(),
                border_subtle: "#cac4d0".into(),
                border_focus: "#6750a4".into(),
                success: "#386a20".into(),
                warning: "#7d5260".into(),
                error: "#b3261e".into(),
                info: "#0061a4".into(),
                shadow: "rgba(0,0,0,0.3)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "12px".into(),
            },
            ThemeVariant::Glassmorphism => AbstractTokens {
                bg_primary: "rgba(255,255,255,0.1)".into(),
                bg_secondary: "rgba(255,255,255,0.05)".into(),
                bg_tertiary: "rgba(255,255,255,0.02)".into(),
                bg_hover: "rgba(255,255,255,0.15)".into(),
                bg_active: "rgba(255,255,255,0.2)".into(),
                bg_disabled: "rgba(255,255,255,0.05)".into(),
                bg_surface: "rgba(255,255,255,0.1)".into(),
                bg_elevated: "rgba(255,255,255,0.15)".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#ffffff".into(),
                text_secondary: "rgba(255,255,255,0.7)".into(),
                text_muted: "rgba(255,255,255,0.5)".into(),
                text_disabled: "rgba(255,255,255,0.3)".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#6366f1".into(),
                accent_secondary: "#818cf8".into(),
                accent_hover: "#7c7cf9".into(),
                accent_subtle: "rgba(99,102,241,0.2)".into(),
                border: "rgba(255,255,255,0.2)".into(),
                border_subtle: "rgba(255,255,255,0.1)".into(),
                border_focus: "#6366f1".into(),
                success: "#22c55e".into(),
                warning: "#f59e0b".into(),
                error: "#ef4444".into(),
                info: "#3b82f6".into(),
                shadow: "rgba(0,0,0,0.3)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "16px".into(),
            },
            ThemeVariant::Claymorphism => AbstractTokens {
                bg_primary: "#f0f4f9".into(),
                bg_secondary: "#e4ebf5".into(),
                bg_tertiary: "#d4dde9".into(),
                bg_hover: "#e8eff7".into(),
                bg_active: "#dce4ef".into(),
                bg_disabled: "#d1d9e6".into(),
                bg_surface: "#f0f4f9".into(),
                bg_elevated: "#ffffff".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#1c1b1f".into(),
                text_secondary: "#49454f".into(),
                text_muted: "#79747e".into(),
                text_disabled: "rgba(28,27,31,0.38)".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#6366f1".into(),
                accent_secondary: "#4f46e5".into(),
                accent_hover: "#7c7cf9".into(),
                accent_subtle: "#e0e7ff".into(),
                border: "rgba(255,255,255,0.4)".into(),
                border_subtle: "rgba(255,255,255,0.2)".into(),
                border_focus: "#6366f1".into(),
                success: "#22c55e".into(),
                warning: "#f59e0b".into(),
                error: "#ef4444".into(),
                info: "#3b82f6".into(),
                shadow:
                    "10px 10px 24px rgba(14,165,233,0.15), -10px -10px 24px rgba(255,255,255,0.8)"
                        .into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "32px".into(),
            },
            ThemeVariant::Skeuomorphism => AbstractTokens {
                bg_primary: "#c0c0c0".into(),
                bg_secondary: "#a8a8a8".into(),
                bg_tertiary: "#909090".into(),
                bg_hover: "#b8b8b8".into(),
                bg_active: "#a0a0a0".into(),
                bg_disabled: "#787878".into(),
                bg_surface: "#d0d0d0".into(),
                bg_elevated: "#e0e0e0".into(),
                bg_backdrop: "rgba(0,0,0,0.6)".into(),
                text_primary: "#1c1b1f".into(),
                text_secondary: "#3c3c3c".into(),
                text_muted: "#666666".into(),
                text_disabled: "#888888".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#808080".into(),
                accent_secondary: "#606060".into(),
                accent_hover: "#909090".into(),
                accent_subtle: "#b0b0b0".into(),
                border: "#666666".into(),
                border_subtle: "#999999".into(),
                border_focus: "#444444".into(),
                success: "#228b22".into(),
                warning: "#daa520".into(),
                error: "#b22222".into(),
                info: "#4169e1".into(),
                shadow: "inset 0 1px 0 rgba(255,255,255,0.9), inset 0 -1px 0 rgba(0,0,0,0.3)"
                    .into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "4px".into(),
            },
            ThemeVariant::NeoBrutalism => AbstractTokens {
                bg_primary: "#ffffff".into(),
                bg_secondary: "#f5f5f5".into(),
                bg_tertiary: "#eeeeee".into(),
                bg_hover: "#fafafa".into(),
                bg_active: "#f0f0f0".into(),
                bg_disabled: "#e0e0e0".into(),
                bg_surface: "#ffffff".into(),
                bg_elevated: "#ffffff".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#000000".into(),
                text_secondary: "#1a1a1a".into(),
                text_muted: "#4a4a4a".into(),
                text_disabled: "#9a9a9a".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#ffde4a".into(),
                accent_secondary: "#ffd700".into(),
                accent_hover: "#ffe066".into(),
                accent_subtle: "#fff3b8".into(),
                border: "#000000".into(),
                border_subtle: "#333333".into(),
                border_focus: "#000000".into(),
                success: "#22c55e".into(),
                warning: "#eab308".into(),
                error: "#ef4444".into(),
                info: "#3b82f6".into(),
                shadow: "4px 4px 0 #000000".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "0px".into(),
            },
            ThemeVariant::Brutalism => AbstractTokens {
                bg_primary: "#ffffff".into(),
                bg_secondary: "#f0f0f0".into(),
                bg_tertiary: "#e0e0e0".into(),
                bg_hover: "#ffffff".into(),
                bg_active: "#f5f5f5".into(),
                bg_disabled: "#d0d0d0".into(),
                bg_surface: "#ffffff".into(),
                bg_elevated: "#ffffff".into(),
                bg_backdrop: "rgba(0,0,0,0.8)".into(),
                text_primary: "#000000".into(),
                text_secondary: "#111111".into(),
                text_muted: "#444444".into(),
                text_disabled: "#888888".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#000000".into(),
                accent_secondary: "#222222".into(),
                accent_hover: "#333333".into(),
                accent_subtle: "#eeeeee".into(),
                border: "#000000".into(),
                border_subtle: "#222222".into(),
                border_focus: "#000000".into(),
                success: "#000000".into(),
                warning: "#000000".into(),
                error: "#000000".into(),
                info: "#000000".into(),
                shadow: "none".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "0px".into(),
            },
            ThemeVariant::Neumorphism => AbstractTokens {
                bg_primary: "#e0e5ec".into(),
                bg_secondary: "#d1d9e6".into(),
                bg_tertiary: "#c8d0e0".into(),
                bg_hover: "#d8dfe9".into(),
                bg_active: "#c8d0e0".into(),
                bg_disabled: "#b0bac6".into(),
                bg_surface: "#e0e5ec".into(),
                bg_elevated: "#e8ecf2".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#1c1b1f".into(),
                text_secondary: "#49454f".into(),
                text_muted: "#79747e".into(),
                text_disabled: "rgba(28,27,31,0.38)".into(),
                text_inverse: "#ffffff".into(),
                accent_primary: "#6366f1".into(),
                accent_secondary: "#4f46e5".into(),
                accent_hover: "#7c7cf9".into(),
                accent_subtle: "#c7d2fe".into(),
                border: "#a0a0a0".into(),
                border_subtle: "#b8bcc8".into(),
                border_focus: "#6366f1".into(),
                success: "#22c55e".into(),
                warning: "#f59e0b".into(),
                error: "#ef4444".into(),
                info: "#3b82f6".into(),
                shadow: "6px 6px 12px rgba(163,177,198,0.6), -6px -6px 12px rgba(255,255,255,0.8)"
                    .into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "20px".into(),
            },
            ThemeVariant::Nord => AbstractTokens {
                bg_primary: "#8CACAE".into(),
                bg_secondary: "#8CACAE".into(),
                bg_tertiary: "#8CACAE".into(),
                bg_hover: "#DDE4EF".into(),
                bg_active: "#D0D8E4".into(),
                bg_disabled: "#E5E9F0".into(),
                bg_surface: "#ECEFF4".into(),
                bg_elevated: "#E5E9F0".into(),
                bg_backdrop: "rgba(0,0,0,0.3)".into(),
                text_primary: "#2E3440".into(),
                text_secondary: "#4C5264".into(),
                text_muted: "#616A7D".into(),
                text_disabled: "#9DA3AE".into(),
                text_inverse: "#ECEFF4".into(),
                accent_primary: "#88C0D0".into(),
                accent_secondary: "#81A1C1".into(),
                accent_hover: "#81A1C1".into(),
                accent_subtle: "#D8DEE9".into(),
                border: "#4C5264".into(),
                border_subtle: "#D8DEE9".into(),
                border_focus: "#88C0D0".into(),
                success: "#A3BE8C".into(),
                warning: "#EBCB8B".into(),
                error: "#BF616A".into(),
                info: "#8FBCBB".into(),
                shadow: "0 2px 8px rgba(46,52,64,0.15)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "8px".into(),
            },
            ThemeVariant::TokyoNight => AbstractTokens {
                bg_primary: "#D5D6DB".into(),
                bg_secondary: "#D5D6DB".into(),
                bg_tertiary: "#D5D6DB".into(),
                bg_hover: "#B5BFDB".into(),
                bg_active: "#AAB3D0".into(),
                bg_disabled: "#C8D0E8".into(),
                bg_surface: "#C0CAF5".into(),
                bg_elevated: "#E0E3F0".into(),
                bg_backdrop: "rgba(26,27,38,0.4)".into(),
                text_primary: "#1A1B26".into(),
                text_secondary: "#565F89".into(),
                text_muted: "#72789A".into(),
                text_disabled: "#9BA4C4".into(),
                text_inverse: "#C0CAF5".into(),
                accent_primary: "#7AA2F7".into(),
                accent_secondary: "#BB9AF7".into(),
                accent_hover: "#BB9AF7".into(),
                accent_subtle: "#BB9AF7".into(),
                border: "#414868".into(),
                border_subtle: "#C0CAF5".into(),
                border_focus: "#7AA2F7".into(),
                success: "#9ECE6A".into(),
                warning: "#E0AF68".into(),
                error: "#F7768E".into(),
                info: "#7DCFFF".into(),
                shadow: "0 4px 12px rgba(26,27,38,0.2)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "6px".into(),
            },
            ThemeVariant::Catppuccin => AbstractTokens {
                bg_primary: "#EFF1F5".into(),
                bg_secondary: "#EFF1F5".into(),
                bg_tertiary: "#EFF1F5".into(),
                bg_hover: "#D8DDE6".into(),
                bg_active: "#D0D5E0".into(),
                bg_disabled: "#E4E7ED".into(),
                bg_surface: "#DCE0E8".into(),
                bg_elevated: "#F5F5F7".into(),
                bg_backdrop: "rgba(76,79,105,0.4)".into(),
                text_primary: "#4C4F69".into(),
                text_secondary: "#6C6F85".into(),
                text_muted: "#9CA0B0".into(),
                text_disabled: "#ACB0BE".into(),
                text_inverse: "#EFF1F5".into(),
                accent_primary: "#CA9EE6".into(),
                accent_secondary: "#F2CDCD".into(),
                accent_hover: "#F2CDCD".into(),
                accent_subtle: "#F5F5F7".into(),
                border: "#BCC0CC".into(),
                border_subtle: "#E8ECF2".into(),
                border_focus: "#CA9EE6".into(),
                success: "#A6E3A1".into(),
                warning: "#F9E2AF".into(),
                error: "#F38BA8".into(),
                info: "#89DCEB".into(),
                shadow: "0 2px 10px rgba(76,79,105,0.12)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "10px".into(),
            },
            ThemeVariant::RosePine => AbstractTokens {
                bg_primary: "#faf4ed".into(),
                bg_secondary: "#faf4ed".into(),
                bg_tertiary: "#faf4ed".into(),
                bg_hover: "#f0ebe3".into(),
                bg_active: "#e8e1d7".into(),
                bg_disabled: "#f5f0e8".into(),
                bg_surface: "#faf4ed".into(),
                bg_elevated: "#fffaf5".into(),
                bg_backdrop: "rgba(87,82,121,0.3)".into(),
                text_primary: "#575279".into(),
                text_secondary: "#6e6a86".into(),
                text_muted: "#9088a4".into(),
                text_disabled: "#b8b2c4".into(),
                text_inverse: "#faf4ed".into(),
                accent_primary: "#ebbcba".into(),
                accent_secondary: "#c4a7e7".into(),
                accent_hover: "#c4a7e7".into(),
                accent_subtle: "#f5efe9".into(),
                border: "#d4c8be".into(),
                border_subtle: "#ebe4da".into(),
                border_focus: "#ebbcba".into(),
                success: "#9ccfd8".into(),
                warning: "#f6c177".into(),
                error: "#ebbcba".into(),
                info: "#31748f".into(),
                shadow: "0 2px 12px rgba(87,82,121,0.1)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "12px".into(),
            },
            ThemeVariant::Linear => AbstractTokens {
                bg_primary: "#FFFFFF".into(),
                bg_secondary: "#FFFFFF".into(),
                bg_tertiary: "#FFFFFF".into(),
                bg_hover: "#EBEBEF".into(),
                bg_active: "#E2E2E8".into(),
                bg_disabled: "#F0F0F2".into(),
                bg_surface: "#F5F5F7".into(),
                bg_elevated: "#FFFFFF".into(),
                bg_backdrop: "rgba(13,13,18,0.4)".into(),
                text_primary: "#0D0D12".into(),
                text_secondary: "#616370".into(),
                text_muted: "#8A8A99".into(),
                text_disabled: "#ADADBC".into(),
                text_inverse: "#FFFFFF".into(),
                accent_primary: "#5E6AD2".into(),
                accent_secondary: "#4650C4".into(),
                accent_hover: "#4650C4".into(),
                accent_subtle: "#EEF0FC".into(),
                border: "#D4D4D8".into(),
                border_subtle: "#EBEBEF".into(),
                border_focus: "#5E6AD2".into(),
                success: "#26A269".into(),
                warning: "#DF9C00".into(),
                error: "#F5547B".into(),
                info: "#0077C8".into(),
                shadow: "0 1px 3px rgba(13,13,18,0.08)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "6px".into(),
            },
        }
    }

    pub fn dark_tokens(&self) -> AbstractTokens {
        match self {
            ThemeVariant::MaterialDesign3 => AbstractTokens {
                bg_primary: "#1c1b1f".into(),
                bg_secondary: "#211f26".into(),
                bg_tertiary: "#2b2930".into(),
                bg_hover: "#2b2930".into(),
                bg_active: "#36343b".into(),
                bg_disabled: "rgba(230,225,229,0.12)".into(),
                bg_surface: "#1c1b1f".into(),
                bg_elevated: "#2b2930".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#e6e1e5".into(),
                text_secondary: "#cac4d0".into(),
                text_muted: "#938f99".into(),
                text_disabled: "rgba(230,225,229,0.38)".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#d0bcff".into(),
                accent_secondary: "#ccc2dc".into(),
                accent_hover: "#b69df8".into(),
                accent_subtle: "#4a4458".into(),
                border: "#938f99".into(),
                border_subtle: "#49454f".into(),
                border_focus: "#d0bcff".into(),
                success: "#4ade80".into(),
                warning: "#f0b8c8".into(),
                error: "#f2b8b5".into(),
                info: "#80d8ff".into(),
                shadow: "rgba(0,0,0,0.5)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "12px".into(),
            },
            ThemeVariant::Glassmorphism => AbstractTokens {
                bg_primary: "rgba(30,30,50,0.6)".into(),
                bg_secondary: "rgba(20,20,40,0.4)".into(),
                bg_tertiary: "rgba(15,15,30,0.3)".into(),
                bg_hover: "rgba(40,40,70,0.5)".into(),
                bg_active: "rgba(50,50,80,0.6)".into(),
                bg_disabled: "rgba(20,20,40,0.3)".into(),
                bg_surface: "rgba(30,30,50,0.6)".into(),
                bg_elevated: "rgba(40,40,70,0.7)".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#e0e0f0".into(),
                text_secondary: "rgba(224,224,240,0.7)".into(),
                text_muted: "rgba(224,224,240,0.5)".into(),
                text_disabled: "rgba(224,224,240,0.3)".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#818cf8".into(),
                accent_secondary: "#a5b4fc".into(),
                accent_hover: "#9399f5".into(),
                accent_subtle: "rgba(129,140,248,0.3)".into(),
                border: "rgba(255,255,255,0.1)".into(),
                border_subtle: "rgba(255,255,255,0.05)".into(),
                border_focus: "#818cf8".into(),
                success: "#4ade80".into(),
                warning: "#fbbf24".into(),
                error: "#f87171".into(),
                info: "#60a5fa".into(),
                shadow: "rgba(0,0,0,0.5)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "16px".into(),
            },
            ThemeVariant::Claymorphism => AbstractTokens {
                bg_primary: "#1a1f2e".into(),
                bg_secondary: "#151a28".into(),
                bg_tertiary: "#101422".into(),
                bg_hover: "#1f2433".into(),
                bg_active: "#181d2a".into(),
                bg_disabled: "#0e1219".into(),
                bg_surface: "#1a1f2e".into(),
                bg_elevated: "#242b3d".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#e6e1e5".into(),
                text_secondary: "#cac4d0".into(),
                text_muted: "#938f99".into(),
                text_disabled: "rgba(230,225,229,0.38)".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#818cf8".into(),
                accent_secondary: "#6366f1".into(),
                accent_hover: "#9399f5".into(),
                accent_subtle: "rgba(99,102,241,0.2)".into(),
                border: "rgba(255,255,255,0.1)".into(),
                border_subtle: "rgba(255,255,255,0.05)".into(),
                border_focus: "#818cf8".into(),
                success: "#4ade80".into(),
                warning: "#fbbf24".into(),
                error: "#f87171".into(),
                info: "#60a5fa".into(),
                shadow: "10px 10px 24px rgba(0,0,0,0.4), -10px -10px 24px rgba(255,255,255,0.05)"
                    .into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "32px".into(),
            },
            ThemeVariant::Skeuomorphism => AbstractTokens {
                bg_primary: "#2a2a2a".into(),
                bg_secondary: "#383838".into(),
                bg_tertiary: "#464646".into(),
                bg_hover: "#323232".into(),
                bg_active: "#404040".into(),
                bg_disabled: "#505050".into(),
                bg_surface: "#303030".into(),
                bg_elevated: "#404040".into(),
                bg_backdrop: "rgba(0,0,0,0.8)".into(),
                text_primary: "#e0e0e0".into(),
                text_secondary: "#b0b0b0".into(),
                text_muted: "#888888".into(),
                text_disabled: "#606060".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#909090".into(),
                accent_secondary: "#707070".into(),
                accent_hover: "#a0a0a0".into(),
                accent_subtle: "#585858".into(),
                border: "#555555".into(),
                border_subtle: "#3a3a3a".into(),
                border_focus: "#777777".into(),
                success: "#32cd32".into(),
                warning: "#ffa500".into(),
                error: "#dc143c".into(),
                info: "#5cacee".into(),
                shadow: "inset 0 1px 0 rgba(255,255,255,0.1), inset 0 -1px 0 rgba(0,0,0,0.5)"
                    .into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "4px".into(),
            },
            ThemeVariant::NeoBrutalism => AbstractTokens {
                bg_primary: "#1a1a1a".into(),
                bg_secondary: "#0f0f0f".into(),
                bg_tertiary: "#050505".into(),
                bg_hover: "#242424".into(),
                bg_active: "#1f1f1f".into(),
                bg_disabled: "#2a2a2a".into(),
                bg_surface: "#1a1a1a".into(),
                bg_elevated: "#242424".into(),
                bg_backdrop: "rgba(0,0,0,0.9)".into(),
                text_primary: "#ffffff".into(),
                text_secondary: "#e0e0e0".into(),
                text_muted: "#a0a0a0".into(),
                text_disabled: "#606060".into(),
                text_inverse: "#000000".into(),
                accent_primary: "#ffde4a".into(),
                accent_secondary: "#ffd700".into(),
                accent_hover: "#ffe066".into(),
                accent_subtle: "#332200".into(),
                border: "#ffffff".into(),
                border_subtle: "#cccccc".into(),
                border_focus: "#ffffff".into(),
                success: "#22c55e".into(),
                warning: "#eab308".into(),
                error: "#ef4444".into(),
                info: "#3b82f6".into(),
                shadow: "4px 4px 0 #ffffff".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "0px".into(),
            },
            ThemeVariant::Brutalism => AbstractTokens {
                bg_primary: "#000000".into(),
                bg_secondary: "#0a0a0a".into(),
                bg_tertiary: "#050505".into(),
                bg_hover: "#000000".into(),
                bg_active: "#0a0a0a".into(),
                bg_disabled: "#1a1a1a".into(),
                bg_surface: "#000000".into(),
                bg_elevated: "#0a0a0a".into(),
                bg_backdrop: "rgba(0,0,0,1)".into(),
                text_primary: "#ffffff".into(),
                text_secondary: "#e0e0e0".into(),
                text_muted: "#a0a0a0".into(),
                text_disabled: "#505050".into(),
                text_inverse: "#000000".into(),
                accent_primary: "#ffffff".into(),
                accent_secondary: "#e0e0e0".into(),
                accent_hover: "#cccccc".into(),
                accent_subtle: "#333333".into(),
                border: "#ffffff".into(),
                border_subtle: "#cccccc".into(),
                border_focus: "#ffffff".into(),
                success: "#4ade80".into(),
                warning: "#fbbf24".into(),
                error: "#f87171".into(),
                info: "#38bdf8".into(),
                shadow: "none".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "0px".into(),
            },
            ThemeVariant::Neumorphism => AbstractTokens {
                bg_primary: "#1e2228".into(),
                bg_secondary: "#171b21".into(),
                bg_tertiary: "#12161c".into(),
                bg_hover: "#1a1f26".into(),
                bg_active: "#161b21".into(),
                bg_disabled: "#1a1d23".into(),
                bg_surface: "#1e2228".into(),
                bg_elevated: "#252b34".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#e0e5ec".into(),
                text_secondary: "#c8d0dc".into(),
                text_muted: "#9aa0a8".into(),
                text_disabled: "rgba(224,229,236,0.38)".into(),
                text_inverse: "#1c1b1f".into(),
                accent_primary: "#6366f1".into(),
                accent_secondary: "#4f46e5".into(),
                accent_hover: "#7c7cf9".into(),
                accent_subtle: "#312e81".into(),
                border: "#3c4350".into(),
                border_subtle: "#2a2f3a".into(),
                border_focus: "#6366f1".into(),
                success: "#4ade80".into(),
                warning: "#fbbf24".into(),
                error: "#f87171".into(),
                info: "#60a5fa".into(),
                shadow: "6px 6px 12px rgba(10,15,25,0.8), -6px -6px 12px rgba(60,70,90,0.3)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "20px".into(),
            },
            ThemeVariant::Nord => AbstractTokens {
                bg_primary: "#2E3440".into(),
                bg_secondary: "#2E3440".into(),
                bg_tertiary: "#2E3440".into(),
                bg_hover: "#434C5E".into(),
                bg_active: "#4C566A".into(),
                bg_disabled: "#2E3440".into(),
                bg_surface: "#3B4252".into(),
                bg_elevated: "#434C5E".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#ECEFF4".into(),
                text_secondary: "#D8DEE9".into(),
                text_muted: "#9DA3AE".into(),
                text_disabled: "#616A7D".into(),
                text_inverse: "#2E3440".into(),
                accent_primary: "#81A1C1".into(),
                accent_secondary: "#88C0D0".into(),
                accent_hover: "#88C0D0".into(),
                accent_subtle: "#3B4252".into(),
                border: "#D8DEE9".into(),
                border_subtle: "#3B4252".into(),
                border_focus: "#88C0D0".into(),
                success: "#A3BE8C".into(),
                warning: "#EBCB8B".into(),
                error: "#BF616A".into(),
                info: "#88C0D0".into(),
                shadow: "0 2px 8px rgba(0,0,0,0.4)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "8px".into(),
            },
            ThemeVariant::TokyoNight => AbstractTokens {
                bg_primary: "#1A1B26".into(),
                bg_secondary: "#1A1B26".into(),
                bg_tertiary: "#1A1B26".into(),
                bg_hover: "#1F2029".into(),
                bg_active: "#292E42".into(),
                bg_disabled: "#1A1B26".into(),
                bg_surface: "#16171F".into(),
                bg_elevated: "#1F2029".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#C0CAF5".into(),
                text_secondary: "#A9B1D6".into(),
                text_muted: "#9AA5CE".into(),
                text_disabled: "#565F89".into(),
                text_inverse: "#C0CAF5".into(),
                accent_primary: "#7AA2F7".into(),
                accent_secondary: "#BB9AF7".into(),
                accent_hover: "#BB9AF7".into(),
                accent_subtle: "#1F2029".into(),
                border: "#3B4261".into(),
                border_subtle: "#1A1B26".into(),
                border_focus: "#7AA2F7".into(),
                success: "#9ECE6A".into(),
                warning: "#E0AF68".into(),
                error: "#F7768E".into(),
                info: "#7DCFFF".into(),
                shadow: "0 4px 16px rgba(0,0,0,0.5)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "6px".into(),
            },
            ThemeVariant::Catppuccin => AbstractTokens {
                bg_primary: "#1E1E2E".into(),
                bg_secondary: "#1E1E2E".into(),
                bg_tertiary: "#1E1E2E".into(),
                bg_hover: "#1E1E2E".into(),
                bg_active: "#313244".into(),
                bg_disabled: "#1E1E2E".into(),
                bg_surface: "#181825".into(),
                bg_elevated: "#1E1E2E".into(),
                bg_backdrop: "rgba(0,0,0,0.6)".into(),
                text_primary: "#CDD6F4".into(),
                text_secondary: "#BAC2DE".into(),
                text_muted: "#A6ADC8".into(),
                text_disabled: "#6C7086".into(),
                text_inverse: "#CDD6F4".into(),
                accent_primary: "#CBA6F7".into(),
                accent_secondary: "#F5E0DC".into(),
                accent_hover: "#F5E0DC".into(),
                accent_subtle: "#1E1E2E".into(),
                border: "#45475A".into(),
                border_subtle: "#181825".into(),
                border_focus: "#CBA6F7".into(),
                success: "#A6E3A1".into(),
                warning: "#F9E2AF".into(),
                error: "#F38BA8".into(),
                info: "#89DCEB".into(),
                shadow: "0 2px 10px rgba(0,0,0,0.4)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "10px".into(),
            },
            ThemeVariant::RosePine => AbstractTokens {
                bg_primary: "#2a2232".into(),
                bg_secondary: "#2a2232".into(),
                bg_tertiary: "#2a2232".into(),
                bg_hover: "#312D3E".into(),
                bg_active: "#3B3546".into(),
                bg_disabled: "#2a2232".into(),
                bg_surface: "#262031".into(),
                bg_elevated: "#312D3E".into(),
                bg_backdrop: "rgba(0,0,0,0.5)".into(),
                text_primary: "#e5e0e8".into(),
                text_secondary: "#cec4d4".into(),
                text_muted: "#a8a0b4".into(),
                text_disabled: "#7a7289".into(),
                text_inverse: "#2a2232".into(),
                accent_primary: "#c4a7e7".into(),
                accent_secondary: "#9ccfd8".into(),
                accent_hover: "#9ccfd8".into(),
                accent_subtle: "#312D3E".into(),
                border: "#40394F".into(),
                border_subtle: "#262031".into(),
                border_focus: "#c4a7e7".into(),
                success: "#9ccfd8".into(),
                warning: "#f6c177".into(),
                error: "#ebbcba".into(),
                info: "#31748f".into(),
                shadow: "0 2px 12px rgba(0,0,0,0.35)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "12px".into(),
            },
            ThemeVariant::Linear => AbstractTokens {
                bg_primary: "#0D0D12".into(),
                bg_secondary: "#0D0D12".into(),
                bg_tertiary: "#0D0D12".into(),
                bg_hover: "#1A1A1F".into(),
                bg_active: "#232329".into(),
                bg_disabled: "#0D0D12".into(),
                bg_surface: "#141419".into(),
                bg_elevated: "#1A1A1F".into(),
                bg_backdrop: "rgba(0,0,0,0.7)".into(),
                text_primary: "#F5F5F7".into(),
                text_secondary: "#A1A1AA".into(),
                text_muted: "#71717A".into(),
                text_disabled: "#52525B".into(),
                text_inverse: "#0D0D12".into(),
                accent_primary: "#9B9CF5".into(),
                accent_secondary: "#B5B8F5".into(),
                accent_hover: "#B5B8F5".into(),
                accent_subtle: "#1A1A1F".into(),
                border: "#2C2C34".into(),
                border_subtle: "#141419".into(),
                border_focus: "#9B9CF5".into(),
                success: "#3EBD93".into(),
                warning: "#F5B631".into(),
                error: "#F76B95".into(),
                info: "#4DA6FF".into(),
                shadow: "0 1px 3px rgba(0,0,0,0.4)".into(),
                shadow_elevation: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -2px rgba(0,0,0,0.1)"
                    .into(),
                text_placeholder: "#79747e".into(),
                radius: "6px".into(),
            },
        }
    }
}

impl AbstractTokens {
    pub fn to_css_vars(&self) -> String {
        let mut css = String::new();
        css.push_str(&format!("--color-bg-primary: {};\n", self.bg_primary));
        css.push_str(&format!("--color-bg-secondary: {};\n", self.bg_secondary));
        css.push_str(&format!("--color-bg-tertiary: {};\n", self.bg_tertiary));
        css.push_str(&format!("--color-bg-hover: {};\n", self.bg_hover));
        css.push_str(&format!("--color-bg-active: {};\n", self.bg_active));
        css.push_str(&format!("--color-bg-disabled: {};\n", self.bg_disabled));
        css.push_str(&format!("--color-bg-surface: {};\n", self.bg_surface));
        css.push_str(&format!("--color-bg-elevated: {};\n", self.bg_elevated));
        css.push_str(&format!("--color-bg-backdrop: {};\n", self.bg_backdrop));
        css.push_str(&format!("--color-text-primary: {};\n", self.text_primary));
        css.push_str(&format!(
            "--color-text-secondary: {};\n",
            self.text_secondary
        ));
        css.push_str(&format!("--color-text-muted: {};\n", self.text_muted));
        css.push_str(&format!("--color-text-disabled: {};\n", self.text_disabled));
        css.push_str(&format!("--color-text-inverse: {};\n", self.text_inverse));
        css.push_str(&format!(
            "--color-accent-primary: {};\n",
            self.accent_primary
        ));
        css.push_str(&format!(
            "--color-accent-secondary: {};\n",
            self.accent_secondary
        ));
        css.push_str(&format!("--color-accent-hover: {};\n", self.accent_hover));
        css.push_str(&format!("--color-accent-subtle: {};\n", self.accent_subtle));
        css.push_str(&format!("--color-border: {};\n", self.border));
        css.push_str(&format!("--color-border-subtle: {};\n", self.border_subtle));
        css.push_str(&format!("--color-border-focus: {};\n", self.border_focus));
        css.push_str(&format!("--color-success: {};\n", self.success));
        css.push_str(&format!("--color-warning: {};\n", self.warning));
        css.push_str(&format!("--color-error: {};\n", self.error));
        css.push_str(&format!("--color-info: {};\n", self.info));
        css.push_str(&format!("--shadow: {};\n", self.shadow));
        css.push_str(&format!("--shadow-elevation: {};\n", self.shadow_elevation));
        css.push_str(&format!(
            "--color-text-placeholder: {};\n",
            self.text_placeholder
        ));
        css.push_str(&format!("--radius: {};\n", self.radius));
        css
    }
}

mod spec;

pub fn generate_css_vars(variant: ThemeVariant, mode: ThemeMode, system_is_dark: bool) -> String {
    let tokens = match mode {
        ThemeMode::Light => variant.light_tokens(),
        ThemeMode::Dark => variant.dark_tokens(),
        ThemeMode::System => {
            if system_is_dark {
                variant.dark_tokens()
            } else {
                variant.light_tokens()
            }
        }
    };
    tokens.to_css_vars()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Theme Loading (name resolution)
    // ---------------------------------------------------------------------------

    #[test]
    fn test_all_theme_variant_names_are_unique() {
        let names: Vec<_> = ThemeVariant::default()
            .to_owned()
            .name()
            .chars()
            .take(1)
            .collect();
        // All names must be non-empty
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::Skeuomorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Brutalism,
            ThemeVariant::Neumorphism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ] {
            assert!(
                !variant.name().is_empty(),
                "Variant {:?} has empty name",
                variant
            );
        }
        // Names are unique
        let all_names: Vec<_> = [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::Skeuomorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Brutalism,
            ThemeVariant::Neumorphism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ]
        .iter()
        .map(|v| v.name())
        .collect();
        let mut sorted = all_names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            all_names.len(),
            sorted.len(),
            "Theme variant names must be unique"
        );
    }

    #[test]
    fn test_all_theme_variant_names_known() {
        let expected = [
            ("material-design-v3", ThemeVariant::MaterialDesign3),
            ("glassmorphism", ThemeVariant::Glassmorphism),
            ("claymorphism", ThemeVariant::Claymorphism),
            ("skeuomorphism", ThemeVariant::Skeuomorphism),
            ("neo-brutalism", ThemeVariant::NeoBrutalism),
            ("brutalism", ThemeVariant::Brutalism),
            ("neumorphism", ThemeVariant::Neumorphism),
            ("nord", ThemeVariant::Nord),
            ("tokyo-night", ThemeVariant::TokyoNight),
            ("catppuccin", ThemeVariant::Catppuccin),
            ("rose-pine", ThemeVariant::RosePine),
            ("linear", ThemeVariant::Linear),
        ];
        for (expected_name, variant) in expected {
            assert_eq!(
                variant.name(),
                expected_name,
                "Variant {:?} should have name '{}'",
                variant,
                expected_name
            );
        }
    }

    // ---------------------------------------------------------------------------
    // CSS Class / Variable Generation
    // ---------------------------------------------------------------------------

    #[test]
    fn test_css_vars_use_double_dash_prefix() {
        let css = generate_css_vars(ThemeVariant::MaterialDesign3, ThemeMode::Light, false);
        // Every CSS variable must use -- prefix (not single -)
        for line in css.lines() {
            if !line.trim().is_empty() {
                assert!(
                    line.contains("--"),
                    "CSS line should contain '--': {}",
                    line
                );
            }
        }
    }

    #[test]
    fn test_css_vars_contain_all_token_fields() {
        let css = generate_css_vars(ThemeVariant::Nord, ThemeMode::Light, false);
        let expected_vars = [
            "--color-bg-primary",
            "--color-bg-surface",
            "--color-bg-elevated",
            "--color-bg-backdrop",
            "--color-text-primary",
            "--color-text-secondary",
            "--color-text-muted",
            "--color-accent-primary",
            "--color-accent-hover",
            "--color-accent-subtle",
            "--color-border",
            "--color-border-subtle",
            "--color-border-focus",
            "--color-success",
            "--color-warning",
            "--color-error",
            "--color-info",
            "--shadow",
            "--radius",
        ];
        for var in expected_vars {
            assert!(
                css.contains(var),
                "CSS should contain variable '{}' but got:\n{}",
                var,
                css
            );
        }
    }

    #[test]
    fn test_css_vars_format_has_semicolons_and_newlines() {
        let css = generate_css_vars(ThemeVariant::Catppuccin, ThemeMode::Dark, false);
        // Every line defining a var should end with semicolon
        for line in css.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            assert!(
                trimmed.ends_with(';'),
                "CSS line should end with ';': {}",
                line
            );
        }
        assert!(
            css.contains('\n'),
            "CSS should contain newlines between declarations"
        );
    }

    // ---------------------------------------------------------------------------
    // Dark / Light / System Mode Switching
    // ---------------------------------------------------------------------------

    #[test]
    fn test_light_mode_produces_light_tokens() {
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ] {
            let css = generate_css_vars(variant, ThemeMode::Light, false);
            let direct = variant.light_tokens().to_css_vars();
            assert_eq!(
                css, direct,
                "Light mode CSS should match direct light_tokens() for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_dark_mode_produces_dark_tokens() {
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ] {
            let css = generate_css_vars(variant, ThemeMode::Dark, false);
            let direct = variant.dark_tokens().to_css_vars();
            assert_eq!(
                css, direct,
                "Dark mode CSS should match direct dark_tokens() for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_system_mode_uses_dark_when_system_is_dark() {
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
        ] {
            let css_dark = generate_css_vars(variant, ThemeMode::System, true);
            let css_light = generate_css_vars(variant, ThemeMode::System, false);
            let dark_tokens = variant.dark_tokens().to_css_vars();
            let light_tokens = variant.light_tokens().to_css_vars();
            assert_eq!(
                css_dark, dark_tokens,
                "System mode with dark system should use dark tokens for {:?}",
                variant
            );
            assert_eq!(
                css_light, light_tokens,
                "System mode with light system should use light tokens for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_light_and_dark_css_are_different() {
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::Skeuomorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Brutalism,
            ThemeVariant::Neumorphism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ] {
            let light_css = generate_css_vars(variant, ThemeMode::Light, false);
            let dark_css = generate_css_vars(variant, ThemeMode::Dark, false);
            assert_ne!(
                light_css, dark_css,
                "Light and dark CSS must differ for {:?}",
                variant
            );
        }
    }

    // ---------------------------------------------------------------------------
    // AbstractTokens field completeness
    // ---------------------------------------------------------------------------

    #[test]
    fn test_light_tokens_all_fields_present_m3() {
        let tokens = ThemeVariant::MaterialDesign3.light_tokens();
        assert_eq!(tokens.bg_primary, "#fefbff");
        assert_eq!(tokens.bg_secondary, "#f7f2fa");
        assert_eq!(tokens.bg_tertiary, "#e8e0ec");
        assert_eq!(tokens.bg_hover, "#e7e0ec");
        assert_eq!(tokens.bg_active, "#e8e0ec");
        assert_eq!(tokens.bg_disabled, "rgba(28,27,31,0.12)");
        assert_eq!(tokens.bg_surface, "#fefbff");
        assert_eq!(tokens.bg_elevated, "#ffffff");
        assert_eq!(tokens.bg_backdrop, "rgba(0,0,0,0.5)");
        assert_eq!(tokens.text_primary, "#1c1b1f");
        assert_eq!(tokens.text_secondary, "#49454f");
        assert_eq!(tokens.text_muted, "#79747e");
        assert_eq!(tokens.text_disabled, "rgba(28,27,31,0.38)");
        assert_eq!(tokens.text_inverse, "#ffffff");
        assert_eq!(tokens.accent_primary, "#6750a4");
        assert_eq!(tokens.accent_secondary, "#625b71");
        assert_eq!(tokens.accent_hover, "#7c67ab");
        assert_eq!(tokens.accent_subtle, "#e8def8");
        assert_eq!(tokens.border, "#79747e");
        assert_eq!(tokens.border_subtle, "#cac4d0");
        assert_eq!(tokens.border_focus, "#6750a4");
        assert_eq!(tokens.success, "#386a20");
        assert_eq!(tokens.warning, "#7d5260");
        assert_eq!(tokens.error, "#b3261e");
        assert_eq!(tokens.info, "#0061a4");
        assert_eq!(tokens.shadow, "rgba(0,0,0,0.3)");
        assert_eq!(tokens.radius, "12px");
    }

    #[test]
    fn test_dark_tokens_all_fields_present_m3() {
        let tokens = ThemeVariant::MaterialDesign3.dark_tokens();
        assert_eq!(tokens.bg_primary, "#1c1b1f");
        assert_eq!(tokens.bg_surface, "#1c1b1f");
        assert_eq!(tokens.bg_elevated, "#2b2930");
        assert_eq!(tokens.text_primary, "#e6e1e5");
        assert_eq!(tokens.accent_primary, "#d0bcff");
        assert_eq!(tokens.success, "#4ade80");
        assert_eq!(tokens.warning, "#f0b8c8");
        assert_eq!(tokens.error, "#f2b8b5");
        assert_eq!(tokens.info, "#80d8ff");
        assert_eq!(tokens.radius, "12px");
    }

    // ---------------------------------------------------------------------------
    // ThemeMode serde + default
    // ---------------------------------------------------------------------------

    #[test]
    fn test_theme_mode_serde() {
        use serde_json;
        for mode in [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System] {
            let json = serde_json::to_string(&mode).unwrap();
            let roundtrip: ThemeMode = serde_json::from_str(&json).unwrap();
            assert_eq!(
                mode, roundtrip,
                "ThemeMode {:?} failed round-trip serde",
                mode
            );
        }
    }

    #[test]
    fn test_theme_mode_default_is_system() {
        let default: ThemeMode = ThemeMode::default();
        assert_eq!(default, ThemeMode::System);
    }

    #[test]
    fn test_theme_variant_default_is_material_design_3() {
        let default: ThemeVariant = ThemeVariant::default();
        assert_eq!(default, ThemeVariant::MaterialDesign3);
    }

    // ---------------------------------------------------------------------------
    // CSS helper method correctness
    // ---------------------------------------------------------------------------

    #[test]
    fn test_css_var_helper_format() {
        // css_var wraps in "var(--...)"
        assert_eq!(
            ThemeVariant::css_var("color-bg-surface"),
            "var(--color-bg-surface)"
        );
    }

    #[test]
    fn test_variant_css_method_helpers() {
        let v = ThemeVariant::MaterialDesign3;
        assert_eq!(v.bg_surface(), "var(--color-bg-surface)");
        assert_eq!(v.bg_elevated(), "var(--color-bg-elevated)");
        assert_eq!(v.bg_backdrop(), "var(--color-bg-backdrop)");
        assert_eq!(v.text_primary(), "var(--color-text-primary)");
        assert_eq!(v.text_secondary(), "var(--color-text-secondary)");
        assert_eq!(v.text_muted(), "var(--color-text-muted)");
        assert_eq!(v.accent_primary(), "var(--color-accent-primary)");
        assert_eq!(v.accent_hover(), "var(--color-accent-hover)");
        assert_eq!(v.accent_subtle(), "var(--color-accent-subtle)");
        assert_eq!(v.text_inverse(), "var(--color-text-inverse)");
        assert_eq!(v.border_default(), "var(--color-border)");
        assert_eq!(v.border_subtle(), "var(--color-border-subtle)");
        assert_eq!(v.border_focus(), "var(--color-border-focus)");
        assert_eq!(v.radius(), "var(--radius)");
        assert_eq!(v.shadow(), "var(--shadow)");
        assert_eq!(v.success(), "var(--color-success)");
        assert_eq!(v.warning(), "var(--color-warning)");
        assert_eq!(v.error(), "var(--color-error)");
    }

    // ---------------------------------------------------------------------------
    // Token value consistency (dark != light for all variants)
    // ---------------------------------------------------------------------------

    #[test]
    fn test_dark_and_light_tokens_differ_for_all_variants() {
        for variant in [
            ThemeVariant::MaterialDesign3,
            ThemeVariant::Glassmorphism,
            ThemeVariant::Claymorphism,
            ThemeVariant::Skeuomorphism,
            ThemeVariant::NeoBrutalism,
            ThemeVariant::Brutalism,
            ThemeVariant::Neumorphism,
            ThemeVariant::Nord,
            ThemeVariant::TokyoNight,
            ThemeVariant::Catppuccin,
            ThemeVariant::RosePine,
            ThemeVariant::Linear,
        ] {
            let light = variant.light_tokens();
            let dark = variant.dark_tokens();
            assert_ne!(
                light.bg_primary, dark.bg_primary,
                "Light/dark bg_primary must differ for {:?}",
                variant
            );
            assert_ne!(
                light.text_primary, dark.text_primary,
                "Light/dark text_primary must differ for {:?}",
                variant
            );
            // bg_surface is the primary surface token and should differ for all variants
            assert_ne!(
                light.bg_surface, dark.bg_surface,
                "Light/dark bg_surface must differ for {:?}",
                variant
            );
        }
    }
}
