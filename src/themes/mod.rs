//! Theme system for Dioxus applications
//!
//! Provides 7-variant theme system with abstract CSS tokens.
//! All components use `--color-*` abstract tokens for cross-variant compatibility.

pub mod tokens;

pub use tokens::{LIGHT_TOKENS, DARK_TOKENS};

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
        }
    }

    pub fn bg_surface(&self, __is_dark: bool) -> &'static str {
        "bg-[var(--color-bg-surface)]"
    }

    pub fn bg_elevated(&self, __is_dark: bool) -> &'static str {
        "bg-[var(--color-bg-elevated)]"
    }

    pub fn bg_backdrop(&self, __is_dark: bool) -> &'static str {
        "bg-[var(--color-bg-backdrop)]"
    }

    pub fn text_primary(&self, __is_dark: bool) -> &'static str {
        "text-[var(--color-text-primary)]"
    }

    pub fn text_secondary(&self, __is_dark: bool) -> &'static str {
        "text-[var(--color-text-secondary)]"
    }

    pub fn text_muted(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-text-muted)]"
    }

    pub fn accent_primary(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-accent-primary)]"
    }

    pub fn accent_hover(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-accent-hover)]"
    }

    pub fn border_default(&self, _is_dark: bool) -> &'static str {
        "border-[var(--color-border)]"
    }

    pub fn border_subtle(&self, _is_dark: bool) -> &'static str {
        "border-[var(--color-border-subtle)]"
    }

    pub fn border_focus(&self, _is_dark: bool) -> &'static str {
        "ring-[var(--color-border-focus)]"
    }

    pub fn radius(&self) -> &'static str {
        "rounded-[var(--radius)]"
    }

    pub fn shadow(&self, _is_dark: bool) -> &'static str {
        "shadow-[var(--shadow)]"
    }

    pub fn success(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-success)]"
    }

    pub fn warning(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-warning)]"
    }

    pub fn error(&self, _is_dark: bool) -> &'static str {
        "text-[var(--color-error)]"
    }
}

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
                shadow: "10px 10px 24px rgba(14,165,233,0.15), -10px -10px 24px rgba(255,255,255,0.8)".into(),
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
                shadow: "inset 0 1px 0 rgba(255,255,255,0.9), inset 0 -1px 0 rgba(0,0,0,0.3)".into(),
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
                shadow: "6px 6px 12px rgba(163,177,198,0.6), -6px -6px 12px rgba(255,255,255,0.8)".into(),
                radius: "20px".into(),
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
                shadow: "10px 10px 24px rgba(0,0,0,0.4), -10px -10px 24px rgba(255,255,255,0.05)".into(),
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
                shadow: "inset 0 1px 0 rgba(255,255,255,0.1), inset 0 -1px 0 rgba(0,0,0,0.5)".into(),
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
                success: "#ffffff".into(),
                warning: "#ffffff".into(),
                error: "#ffffff".into(),
                info: "#ffffff".into(),
                shadow: "none".into(),
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
                radius: "20px".into(),
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
        css.push_str(&format!("--color-text-secondary: {};\n", self.text_secondary));
        css.push_str(&format!("--color-text-muted: {};\n", self.text_muted));
        css.push_str(&format!("--color-text-disabled: {};\n", self.text_disabled));
        css.push_str(&format!("--color-text-inverse: {};\n", self.text_inverse));
        css.push_str(&format!("--color-accent-primary: {};\n", self.accent_primary));
        css.push_str(&format!("--color-accent-secondary: {};\n", self.accent_secondary));
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
        css.push_str(&format!("--radius: {};\n", self.radius));
        css
    }
}

pub fn generate_css_vars(variant: ThemeVariant, mode: ThemeMode) -> String {
    let tokens = match mode {
        ThemeMode::Light => variant.light_tokens(),
        ThemeMode::Dark => variant.dark_tokens(),
        ThemeMode::System => variant.light_tokens(),
    };
    tokens.to_css_vars()
}

pub fn theme_variant_from_str(s: &str) -> Option<ThemeVariant> {
    match s {
        "material-design-v3" | "m3" | "material" => Some(ThemeVariant::MaterialDesign3),
        "glassmorphism" | "glass" => Some(ThemeVariant::Glassmorphism),
        "claymorphism" | "clay" => Some(ThemeVariant::Claymorphism),
        "skeuomorphism" | "skeuo" => Some(ThemeVariant::Skeuomorphism),
        "neo-brutalism" | "neobrut" | "neo" => Some(ThemeVariant::NeoBrutalism),
        "brutalism" | "brut" => Some(ThemeVariant::Brutalism),
        "neumorphism" | "neu" => Some(ThemeVariant::Neumorphism),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m3_light_tokens() {
        let tokens = ThemeVariant::MaterialDesign3.light_tokens();
        assert_eq!(tokens.bg_primary, "#fefbff");
        assert_eq!(tokens.accent_primary, "#6750a4");
        assert_eq!(tokens.radius, "12px");
    }

    #[test]
    fn test_m3_dark_tokens() {
        let tokens = ThemeVariant::MaterialDesign3.dark_tokens();
        assert_eq!(tokens.bg_primary, "#1c1b1f");
        assert_eq!(tokens.accent_primary, "#d0bcff");
    }

    #[test]
    fn test_glass_tokens() {
        let tokens = ThemeVariant::Glassmorphism.light_tokens();
        assert!(tokens.bg_primary.starts_with("rgba"));
        assert_eq!(tokens.accent_primary, "#6366f1");
        assert_eq!(tokens.radius, "16px");
    }

    #[test]
    fn test_clay_tokens() {
        let tokens = ThemeVariant::Claymorphism.light_tokens();
        assert!(tokens.shadow.contains("rgba"));
        assert_eq!(tokens.radius, "32px");
    }

    #[test]
    fn test_neo_brutal_tokens() {
        let tokens = ThemeVariant::NeoBrutalism.light_tokens();
        assert_eq!(tokens.accent_primary, "#ffde4a");
        assert_eq!(tokens.border, "#000000");
        assert_eq!(tokens.radius, "0px");
        assert!(tokens.shadow.contains("4px"));
    }

    #[test]
    fn test_abstract_tokens_to_css() {
        let tokens = ThemeVariant::MaterialDesign3.light_tokens();
        let css = tokens.to_css_vars();
        assert!(css.contains("--color-bg-primary:"));
        assert!(css.contains("--color-accent-primary:"));
    }

    #[test]
    fn test_generate_css_vars_light() {
        let css = generate_css_vars(ThemeVariant::MaterialDesign3, ThemeMode::Light);
        assert!(css.contains("#fefbff"));
    }

    #[test]
    fn test_generate_css_vars_dark() {
        let css = generate_css_vars(ThemeVariant::MaterialDesign3, ThemeMode::Dark);
        assert!(css.contains("#1c1b1f"));
    }

    #[test]
    fn test_theme_variant_name() {
        assert_eq!(ThemeVariant::MaterialDesign3.name(), "material-design-v3");
        assert_eq!(ThemeVariant::Glassmorphism.name(), "glassmorphism");
        assert_eq!(ThemeVariant::NeoBrutalism.name(), "neo-brutalism");
    }

    #[test]
    fn test_theme_variant_from_str() {
        assert_eq!(theme_variant_from_str("material-design-v3"), Some(ThemeVariant::MaterialDesign3));
        assert_eq!(theme_variant_from_str("glass"), Some(ThemeVariant::Glassmorphism));
        assert_eq!(theme_variant_from_str("neo"), Some(ThemeVariant::NeoBrutalism));
        assert_eq!(theme_variant_from_str("unknown"), None);
    }
}
