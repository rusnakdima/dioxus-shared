//! Flowbite component class resolution
//!
//! Maps SDUI component names and variants to theme-appropriate TailwindCSS classes.
//! Used by the DynamicRenderer to apply correct styling based on the active theme.

use crate::themes::ThemeVariant;

// ===== Public API =====

/// Resolves Flowbite/TailwindCSS classes for a named component.
///
/// Routes to the appropriate `resolve_*_classes` helper based on component name.
pub fn resolve_flowbite_classes(
    component: &str,
    component_variant: Option<&str>,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    let component_variant = component_variant.unwrap_or("default");

    match component {
        "button" | "action-button" => {
            resolve_button_classes(component_variant, theme_variant, is_dark)
        }
        "input" | "action-input" => {
            resolve_input_classes(component_variant, theme_variant, is_dark)
        }
        "textarea" | "action-textarea" => {
            resolve_textarea_classes(component_variant, theme_variant, is_dark)
        }
        "select" | "action-select" => {
            resolve_select_classes(component_variant, theme_variant, is_dark)
        }
        "text" => resolve_text_classes(component_variant, theme_variant, is_dark),
        "badge" => resolve_badge_classes(component_variant, theme_variant, is_dark),
        "chip" => resolve_chip_classes(component_variant, theme_variant, is_dark),
        "switch" => resolve_switch_classes(component_variant, theme_variant, is_dark),
        "tabs" => resolve_tabs_classes(component_variant, theme_variant, is_dark),
        "avatar" => resolve_avatar_classes(component_variant, theme_variant, is_dark),
        "icon" => resolve_icon_classes(component_variant, theme_variant, is_dark),
        "image" => resolve_image_classes(component_variant, theme_variant, is_dark),
        "card" => resolve_card_classes(component_variant, theme_variant, is_dark),
        "div" => resolve_div_classes(),
        "header" => resolve_header_classes(component_variant, theme_variant, is_dark),
        "sidebar" => resolve_sidebar_classes(component_variant, theme_variant, is_dark),
        "spacer" => resolve_spacer_classes(),
        "divider" => resolve_divider_classes(component_variant, theme_variant, is_dark),
        "tooltip" => resolve_tooltip_classes(component_variant, theme_variant, is_dark),
        "modal" => resolve_modal_classes(component_variant, theme_variant, is_dark),
        _ => resolve_surface_classes(theme_variant, is_dark),
    }
}

// ===== Button =====

fn get_button_base(component_variant: &str, theme_variant: ThemeVariant, is_dark: bool) -> String {
    match theme_variant {
        ThemeVariant::MaterialDesign3 => {
            match component_variant {
                "primary" => "",
                "secondary" => "",
                "outline" => "border-2 ",
                "ghost" => "",
                "danger" => "",
                "success" => "",
                "warning" => "",
                "gradient" => "",
                _ => "",
            }
        }
        ThemeVariant::Glassmorphism => {
            if is_dark {
                match component_variant {
                    "primary" => "backdrop-blur-md border shadow-lg",
                    "secondary" => "backdrop-blur-md border ",
                    "outline" => "bg-transparent backdrop-blur-md border ",
                    "ghost" => "backdrop-blur-md ",
                    "danger" => "backdrop-blur-md ",
                    "success" => "backdrop-blur-md ",
                    _ => "backdrop-blur-md border shadow-lg",
                }
            } else {
                match component_variant {
                    "primary" => "backdrop-blur-md border shadow-xl",
                    "secondary" => "backdrop-blur-md border ",
                    "outline" => "bg-transparent backdrop-blur-md border ",
                    "ghost" => "backdrop-blur-md ",
                    "danger" => "backdrop-blur-md ",
                    "success" => "backdrop-blur-md ",
                    _ => "backdrop-blur-md border shadow-xl",
                }
            }
        }
        ThemeVariant::Claymorphism => {
            if is_dark {
                match component_variant {
                    "primary" => "bg-gradient-to-br from-gray-700 to-gray-800 shadow-[8px_8px_16px_#1a1a1a,-8px_-8px_16px_#2a2a2a]",
                    "secondary" => "bg-gradient-to-br from-gray-800 to-gray-900 shadow-[6px_6px_12px_#1a1a1a,-6px_-6px_12px_#252525]",
                    "outline" => "bg-gradient-to-br from-gray-800 to-gray-900 shadow-[6px_6px_12px_#1a1a1a,-6px_-6px_12px_#252525] ",
                    "ghost" => "",
                    _ => "bg-gradient-to-br shadow-[8px_8px_16px_#1a1a1a,-8px_-8px_16px_#2a2a2a]",
                }
            } else {
                match component_variant {
                    "primary" => "bg-gradient-to-br shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff]",
                    "secondary" => "bg-gradient-to-br shadow-[6px_6px_12px_#d1d5db,-6px_-6px_12px_#ffffff]",
                    "outline" => "bg-gradient-to-br shadow-[6px_6px_12px_#d1d5db,-6px_-6px_12px_#ffffff] ",
                    "ghost" => "",
                    _ => "bg-gradient-to-br shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff]",
                }
            }
        }
        ThemeVariant::Skeuomorphism => {
            if is_dark {
                match component_variant {
                    "primary" => "bg-gradient-to-b from-gray-600 to-gray-700 shadow-[inset_0_1px_0_rgba(255,255,255,0.1),0_2px_4px_rgba(0,0,0,0.3)]",
                    "secondary" => "bg-gradient-to-b from-gray-700 to-gray-800 shadow-[inset_0_1px_0_rgba(255,255,255,0.05),0_2px_4px_rgba(0,0,0,0.4)]",
                    "outline" => "bg-gradient-to-b from-gray-800 to-gray-900 shadow-[inset_0_1px_0_rgba(255,255,255,0.05),0_2px_4px_rgba(0,0,0,0.4)] ",
                    "ghost" => "",
                    _ => "bg-gradient-to-b shadow-[inset_0_1px_0_rgba(255,255,255,0.1),0_2px_4px_rgba(0,0,0,0.3)]",
                }
            } else {
                match component_variant {
                    "primary" => "bg-gradient-to-b shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_2px_4px_rgba(0,0,0,0.2)]",
                    "secondary" => "bg-gradient-to-b shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_2px_4px_rgba(0,0,0,0.15)]",
                    "outline" => "bg-gradient-to-b shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_2px_4px_rgba(0,0,0,0.1)] ",
                    "ghost" => "",
                    _ => "bg-gradient-to-b shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_2px_4px_rgba(0,0,0,0.2)]",
                }
            }
        }
        ThemeVariant::NeoBrutalism => {
            if is_dark {
                match component_variant {
                    "primary" => "border-2 shadow-[4px_4px_0px_#000]",
                    "secondary" => "border-2 shadow-[4px_4px_0px_#fff]",
                    "outline" => "bg-transparent border-2 shadow-[4px_4px_0px_#000]",
                    "ghost" => "",
                    "danger" => "border-2 shadow-[4px_4px_0px_#000]",
                    "success" => "border-2 shadow-[4px_4px_0px_#000]",
                    _ => "border-2 shadow-[4px_4px_0px_#000]",
                }
            } else {
                match component_variant {
                    "primary" => "border-2 shadow-[4px_4px_0px_#000]",
                    "secondary" => "border-2 shadow-[4px_4px_0px_#000]",
                    "outline" => "bg-transparent border-2 shadow-[4px_4px_0px_#000]",
                    "ghost" => "",
                    "danger" => "border-2 shadow-[4px_4px_0px_#000]",
                    "success" => "border-2 shadow-[4px_4px_0px_#000]",
                    _ => "border-2 shadow-[4px_4px_0px_#000]",
                }
            }
        }
        ThemeVariant::Brutalism => {
            match component_variant {
                "primary" => "border-2 rounded-none p-4",
                "secondary" => "border-2 rounded-none p-4",
                "outline" => "bg-transparent border-2 rounded-none",
                "ghost" => "rounded-none",
                _ => "border-2 rounded-none p-4",
            }
        }
        ThemeVariant::Neumorphism => {
            if is_dark {
                match component_variant {
                    "primary" => "shadow-[8px_8px_16px_#0d0d0d,-8px_-8px_16px_#1a1a1a]",
                    "secondary" => "shadow-[8px_8px_16px_#0d0d0d,-8px_-8px_16px_#1a1a1a]",
                    "outline" => "shadow-[8px_8px_16px_#0d0d0d,-8px_-8px_16px_#1a1a1a] ",
                    "ghost" => "shadow-[4px_4px_8px_#0d0d0d,-4px_-4px_8px_#1a1a1a]",
                    _ => "shadow-[8px_8px_16px_#0d0d0d,-8px_-8px_16px_#1a1a1a]",
                }
            } else {
                match component_variant {
                    "primary" => "shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff]",
                    "secondary" => "shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff]",
                    "outline" => "shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff] ",
                    "ghost" => "shadow-[4px_4px_8px_#d1d5db,-4px_-4px_8px_#ffffff]",
                    _ => "shadow-[8px_8px_16px_#d1d5db,-8px_-8px_16px_#ffffff]",
                }
            }
        }
        ThemeVariant::Nord => {
            match component_variant {
                "primary" => "",
                "secondary" => "",
                "outline" => "border ",
                "ghost" => "",
                "danger" => "",
                "success" => "",
                _ => "",
            }
        }
        ThemeVariant::TokyoNight => {
            match component_variant {
                "primary" => "",
                "secondary" => "",
                "outline" => "border ",
                "ghost" => "",
                "danger" => "",
                "success" => "",
                _ => "",
            }
        }
        ThemeVariant::Catppuccin => {
            match component_variant {
                "primary" => "rounded-lg",
                "secondary" => "",
                "outline" => "border rounded-lg",
                "ghost" => "rounded-lg",
                "danger" => "",
                "success" => "",
                _ => "rounded-lg",
            }
        }
        ThemeVariant::RosePine => {
            match component_variant {
                "primary" => "rounded-lg",
                "secondary" => "",
                "outline" => "border rounded-lg",
                "ghost" => "rounded-lg",
                "danger" => "",
                "success" => "",
                _ => "rounded-lg",
            }
        }
        ThemeVariant::Linear => {
            match component_variant {
                "primary" => "rounded-full",
                "secondary" => "",
                "outline" => "border rounded-full",
                "ghost" => "rounded-full",
                "danger" => "",
                "success" => "",
                _ => "rounded-full",
            }
        }
    }.to_string()
}

fn resolve_button_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    let base = get_button_base(component_variant, theme_variant, is_dark);
    format!("{} px-5 py-2.5 font-medium transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed", base)
}

// ===== Input =====

fn resolve_input_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    let base = match (theme_variant, component_variant, is_dark) {
        (ThemeVariant::MaterialDesign3, "filled", _)
        | (ThemeVariant::Nord, "filled", _)
        | (ThemeVariant::TokyoNight, "filled", _)
        | (ThemeVariant::Catppuccin, "filled", _)
        | (ThemeVariant::RosePine, "filled", _)
        | (ThemeVariant::Linear, "filled", _) => "w-full px-4 py-3 border-0 ",
        (ThemeVariant::MaterialDesign3, "outline", true)
        | (ThemeVariant::Nord, "outline", true)
        | (ThemeVariant::TokyoNight, "outline", true)
        | (ThemeVariant::Catppuccin, "outline", true)
        | (ThemeVariant::RosePine, "outline", true)
        | (ThemeVariant::Linear, "outline", true) => {
            "w-full px-4 py-3 border bg-transparent focus:ring-1 "
        }
        (ThemeVariant::MaterialDesign3, "outline", false)
        | (ThemeVariant::Nord, "outline", false)
        | (ThemeVariant::TokyoNight, "outline", false)
        | (ThemeVariant::Catppuccin, "outline", false)
        | (ThemeVariant::RosePine, "outline", false)
        | (ThemeVariant::Linear, "outline", false) => "w-full px-4 py-3 border focus:ring-1 ",
        (
            ThemeVariant::MaterialDesign3
            | ThemeVariant::Nord
            | ThemeVariant::TokyoNight
            | ThemeVariant::Catppuccin
            | ThemeVariant::RosePine
            | ThemeVariant::Linear,
            _,
            _,
        ) => "w-full px-4 py-3 border focus:ring-1 ",
        (ThemeVariant::Glassmorphism, _, _) => "w-full px-4 py-3 border backdrop-blur-md ",
        (ThemeVariant::Claymorphism, _, _) => "w-full px-4 py-3 border ",
        (ThemeVariant::Skeuomorphism, _, _) => "w-full px-4 py-3 border ",
        (ThemeVariant::NeoBrutalism, _, _) => "w-full px-4 py-3 border-2 ",
        (ThemeVariant::Brutalism, _, _) => "w-full px-4 py-3 rounded-none border-2 ",
        (ThemeVariant::Neumorphism, _, _) => "w-full px-4 py-3 border ",
    }
    .to_string();

    format!("{base} transition-colors text-sm")
}

fn resolve_textarea_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    resolve_input_classes(component_variant, theme_variant, is_dark)
}

fn resolve_select_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    resolve_input_classes(component_variant, theme_variant, is_dark)
}

// ===== Text =====

fn resolve_text_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    match theme_variant {
        ThemeVariant::MaterialDesign3 => match component_variant {
            "primary" => {
                if is_dark {
                    "text-white"
                } else {
                    "text-gray-900"
                }
            }
            "secondary" => {
                if is_dark {
                    "text-gray-300"
                } else {
                    "text-gray-600"
                }
            }
            "muted" => {
                if is_dark {
                    "text-gray-400"
                } else {
                    "text-gray-500"
                }
            }
            "danger" => "text-red-600",
            "success" => "text-green-600",
            "warning" => "text-yellow-600",
            _ => {
                if is_dark {
                    "text-white"
                } else {
                    "text-gray-900"
                }
            }
        }
        .to_string(),
        ThemeVariant::Glassmorphism => if is_dark {
            "text-white"
        } else {
            "text-gray-900"
        }
        .to_string(),
        ThemeVariant::Neumorphism => if is_dark {
            "text-gray-200"
        } else {
            "text-gray-700"
        }
        .to_string(),
        _ => if is_dark {
            "text-white"
        } else {
            "text-gray-900"
        }
        .to_string(),
    }
}

// ===== Badge =====

fn resolve_badge_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let base = match theme_variant {
        ThemeVariant::MaterialDesign3 => match component_variant {
            "primary" => "rounded-full",
            "secondary" => "rounded-full",
            "success" => "rounded-full",
            "warning" => "rounded-full",
            "danger" => "rounded-full",
            "outline" => "rounded-full border ",
            _ => "rounded-full",
        },
        ThemeVariant::Glassmorphism => "backdrop-blur-md rounded-full border ",
        ThemeVariant::Claymorphism => "rounded-full ",
        ThemeVariant::NeoBrutalism => "rounded-lg border-2 px-3 py-1",
        ThemeVariant::Brutalism => "border-2 px-3 py-1",
        ThemeVariant::Neumorphism => "rounded-full ",
        ThemeVariant::Nord => match component_variant {
            "primary" => "",
            "secondary" => "",
            "success" => "",
            _ => "",
        },
        ThemeVariant::Linear => "rounded-full",
        _ => match component_variant {
            "primary" => "",
            "secondary" => "",
            "success" => "",
            "warning" => "",
            "danger" => "",
            "outline" => "border ",
            _ => "",
        },
    }
    .to_string();

    format!(
        "{} inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium",
        base
    )
}

// ===== Chip =====

fn resolve_chip_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let base = match theme_variant {
        ThemeVariant::MaterialDesign3 => match component_variant {
            "primary" => "border ",
            "secondary" => "border ",
            "success" => "border ",
            "warning" => "border ",
            "danger" => "border ",
            "outline" => "border ",
            _ => "border ",
        },
        ThemeVariant::Glassmorphism => "backdrop-blur-md rounded-full border ",
        ThemeVariant::NeoBrutalism => "border-2 ",
        _ => match component_variant {
            "primary" => "border ",
            "secondary" => "border ",
            "outline" => "border ",
            _ => "border ",
        },
    }
    .to_string();

    format!(
        "{} inline-flex items-center px-3 py-1.5 rounded-full text-sm font-medium",
        base
    )
}

// ===== Switch =====

fn resolve_switch_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let (base, track) = match theme_variant {
        ThemeVariant::Glassmorphism => {
            let track = "";
            let base = match component_variant {
                "primary" => "",
                "success" => "",
                "danger" => "",
                _ => "",
            };
            (base, track)
        }
        ThemeVariant::NeoBrutalism => {
            let track = "";
            let base = match component_variant {
                "primary" => "border-2 ",
                "success" => "border-2 ",
                _ => "border-2 ",
            };
            (base, track)
        }
        ThemeVariant::Neumorphism => {
            let track = "";
            let base = match component_variant {
                "primary" => "",
                _ => "",
            };
            (base, track)
        }
        _ => {
            let track = "";
            let base = match component_variant {
                "primary" => "",
                "secondary" => "",
                "success" => "",
                "warning" => "",
                "danger" => "",
                _ => "",
            };
            (base, track)
        }
    };
    format!(
        "{} {} relative inline-flex h-6 w-11 items-center rounded-full transition-colors",
        base, track
    )
}

// ===== Tabs =====

fn resolve_tabs_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let base = match theme_variant {
        ThemeVariant::Glassmorphism => match component_variant {
            "primary" => "",
            _ => "",
        },
        ThemeVariant::NeoBrutalism => match component_variant {
            "primary" => "",
            _ => "",
        },
        ThemeVariant::Neumorphism => match component_variant {
            "primary" => "",
            _ => "",
        },
        _ => match component_variant {
            "primary" => "",
            "secondary" => "",
            "success" => "",
            "warning" => "",
            "danger" => "",
            _ => "",
        },
    }
    .to_string();

    format!(
        "{} inline-flex items-center border-b-2 px-4 py-2 text-sm font-medium transition-colors",
        base
    )
}

// ===== Avatar =====

fn resolve_avatar_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let shape = match component_variant {
        "circle" => "rounded-full",
        "square" => "rounded-none",
        _ => "rounded-lg",
    };

    let (bg, _text) = match theme_variant {
        ThemeVariant::Glassmorphism => ("backdrop-blur-md".to_string(), "".to_string()),
        ThemeVariant::NeoBrutalism => ("".to_string(), "".to_string()),
        ThemeVariant::Neumorphism => ("".to_string(), "".to_string()),
        _ => ("".to_string(), "".to_string()),
    };

    format!(
        "{} w-10 h-10 {} flex items-center justify-center",
        shape, bg
    )
}

// ===== Icon =====

fn resolve_icon_classes(
    _component_variant: &str,
    _theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    "w-5 h-5 ".to_string()
}

// ===== Image =====

fn resolve_image_classes(
    _component_variant: &str,
    _theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    "w-full h-auto rounded-lg object-cover".to_string()
}

// ===== Card =====

fn resolve_card_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let base = match theme_variant {
        ThemeVariant::MaterialDesign3 => match component_variant {
            "elevated" => "rounded-lg border p-5",
            "flat" => "rounded-lg border p-5",
            "outline" => "rounded-lg bg-transparent border-2 p-5",
            _ => "rounded-lg border p-5",
        },
        ThemeVariant::Glassmorphism => "backdrop-blur-md border p-5",
        ThemeVariant::Claymorphism => "p-5",
        ThemeVariant::Skeuomorphism => "border p-5",
        ThemeVariant::NeoBrutalism => "border-2 p-5",
        ThemeVariant::Brutalism => "rounded-none border-2 p-5",
        ThemeVariant::Neumorphism => "p-5",
        ThemeVariant::Nord => "border p-5",
        ThemeVariant::Linear => "border p-5",
        _ => match component_variant {
            "elevated" => "border p-5",
            _ => "border p-5",
        },
    }
    .to_string();

    format!("{base} transition-shadow hover:shadow-md")
}

// ===== Div =====

fn resolve_div_classes() -> String {
    "flex flex-col".to_string()
}

// ===== Header =====

fn resolve_header_classes(
    _component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    match theme_variant {
        ThemeVariant::Glassmorphism => "backdrop-blur-md border-b px-6 py-4".to_string(),
        ThemeVariant::NeoBrutalism => "border-b-2 px-6 py-4".to_string(),
        ThemeVariant::Brutalism => "border-b-2 px-6 py-4".to_string(),
        _ => "border-b px-6 py-4".to_string(),
    }
}

// ===== Sidebar =====

fn resolve_sidebar_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let width = match component_variant {
        "collapsed" => "w-16",
        "rail" => "w-20",
        _ => "w-64",
    };

    let (bg, border) = match theme_variant {
        ThemeVariant::Glassmorphism => ("backdrop-blur-md".to_string(), "border-r ".to_string()),
        ThemeVariant::NeoBrutalism => ("".to_string(), "border-r-2 ".to_string()),
        _ => ("".to_string(), "border-r ".to_string()),
    };

    format!("{width} {bg} {border} h-full overflow-y-auto")
}

// ===== Spacer =====

fn resolve_spacer_classes() -> String {
    "flex-1".to_string()
}

// ===== Divider =====

fn resolve_divider_classes(
    _component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    match theme_variant {
        ThemeVariant::Glassmorphism => "border-t my-4".to_string(),
        ThemeVariant::NeoBrutalism => "border-t-2 my-4".to_string(),
        ThemeVariant::Brutalism => "border-t-2 my-4".to_string(),
        _ => "border-t my-4".to_string(),
    }
}

// ===== Tooltip =====

fn resolve_tooltip_classes(
    _component_variant: &str,
    theme_variant: ThemeVariant,
    is_dark: bool,
) -> String {
    match theme_variant {
        ThemeVariant::Glassmorphism => {
            "backdrop-blur-md text-xs px-3 py-1.5 rounded-full border ".to_string()
        }
        ThemeVariant::NeoBrutalism => "text-xs px-3 py-1.5 rounded-lg border-2".to_string(),
        ThemeVariant::Neumorphism => "text-xs px-3 py-1.5 rounded-xl ".to_string(),
        _ => {
            if is_dark {
                "text-xs px-2 py-1 rounded border ".to_string()
            } else {
                "text-xs px-2 py-1 rounded ".to_string()
            }
        }
    }
}

// ===== Modal =====

fn resolve_modal_classes(
    component_variant: &str,
    theme_variant: ThemeVariant,
    _is_dark: bool,
) -> String {
    let base = match theme_variant {
        ThemeVariant::Glassmorphism => match component_variant {
            "fullscreen" => "fixed inset-0 z-50 backdrop-blur-xl",
            "alert" | "confirm" => "backdrop-blur-xl border p-6 max-w-md mx-auto mt-20 ",
            _ => "backdrop-blur-xl border p-6 max-w-2xl mx-auto mt-20 ",
        },
        ThemeVariant::NeoBrutalism => match component_variant {
            "fullscreen" => "fixed inset-0 z-50 ",
            "alert" | "confirm" => "border-2 p-6 max-w-md mx-auto mt-20",
            _ => "border-2 p-6 max-w-2xl mx-auto mt-20",
        },
        ThemeVariant::Brutalism => match component_variant {
            "fullscreen" => "fixed inset-0 z-50 ",
            "alert" | "confirm" => "rounded-none border-2 p-6 max-w-md mx-auto mt-20",
            _ => "rounded-none border-2 p-6 max-w-2xl mx-auto mt-20",
        },
        ThemeVariant::Neumorphism => match component_variant {
            "fullscreen" => "fixed inset-0 z-50 ",
            "alert" | "confirm" => "p-6 max-w-md mx-auto mt-20",
            _ => "p-6 max-w-2xl mx-auto mt-20",
        },
        _ => match component_variant {
            "fullscreen" => "fixed inset-0 z-50 ",
            "alert" | "confirm" => "border p-6 max-w-md mx-auto mt-20",
            _ => "border p-6 max-w-2xl mx-auto mt-20",
        },
    }
    .to_string();

    format!("{base} relative z-50")
}

// ===== Surface =====

fn resolve_surface_classes(theme_variant: ThemeVariant, is_dark: bool) -> String {
    match theme_variant {
        ThemeVariant::MaterialDesign3 => {
            if is_dark {
                "bg-gray-800 text-white rounded".to_string()
            } else {
                "bg-white text-gray-900 rounded".to_string()
            }
        }
        ThemeVariant::Glassmorphism => "backdrop-blur-md bg-white/50 border".to_string(),
        ThemeVariant::NeoBrutalism => "border-2 bg-white".to_string(),
        ThemeVariant::Brutalism => "border-2 bg-white".to_string(),
        ThemeVariant::Neumorphism => "bg-gray-200 rounded-lg".to_string(),
        _ => {
            if is_dark {
                "bg-gray-800 text-white rounded".to_string()
            } else {
                "bg-white text-gray-900 rounded".to_string()
            }
        }
    }
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::ThemeVariant;

    // === resolve_flowbite_classes mock-data tests ===

    #[test]
    fn resolve_flowbite_button_primary_light_m3() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains("px-5"));
        assert!(classes.contains("py-2.5"));
    }

    #[test]
    fn resolve_flowbite_button_primary_dark_m3() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            true,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_button_danger_light() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("danger"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_button_outline_glassmorphism_dark() {
        let classes =
            resolve_flowbite_classes("button", Some("outline"), ThemeVariant::Glassmorphism, true);
        assert!(classes.contains("backdrop-blur-md"));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_button_gradient_light() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("gradient"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_input_outline_light() {
        let classes = resolve_flowbite_classes(
            "input",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("w-full"));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_input_filled_dark() {
        let classes =
            resolve_flowbite_classes("input", Some("filled"), ThemeVariant::MaterialDesign3, true);
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_text_light() {
        let classes = resolve_flowbite_classes("text", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_text_dark() {
        let classes = resolve_flowbite_classes("text", None, ThemeVariant::MaterialDesign3, true);
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_text_glassmorphism_light() {
        let classes = resolve_flowbite_classes("text", None, ThemeVariant::Glassmorphism, false);
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_text_glassmorphism_dark() {
        let classes = resolve_flowbite_classes("text", None, ThemeVariant::Glassmorphism, true);
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_badge_success_light() {
        let classes = resolve_flowbite_classes(
            "badge",
            Some("success"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_badge_success_dark() {
        let classes = resolve_flowbite_classes(
            "badge",
            Some("success"),
            ThemeVariant::MaterialDesign3,
            true,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_chip_primary_light() {
        let classes = resolve_flowbite_classes(
            "chip",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains("rounded-full"));
    }

    #[test]
    fn resolve_flowbite_switch_primary_light() {
        let classes = resolve_flowbite_classes(
            "switch",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains("")); // track
        assert!(classes.contains("rounded-full"));
    }

    #[test]
    fn resolve_flowbite_switch_primary_dark() {
        let classes = resolve_flowbite_classes(
            "switch",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            true,
        );
        assert!(classes.contains(""));
        assert!(classes.contains("")); // track
    }

    #[test]
    fn resolve_flowbite_card_elevated_light() {
        let classes = resolve_flowbite_classes(
            "card",
            Some("elevated"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains("p-5"));
    }

    #[test]
    fn resolve_flowbite_card_elevated_dark() {
        let classes = resolve_flowbite_classes(
            "card",
            Some("elevated"),
            ThemeVariant::MaterialDesign3,
            true,
        );
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_card_glassmorphism_light() {
        let classes = resolve_flowbite_classes("card", None, ThemeVariant::Glassmorphism, false);
        assert!(classes.contains(""));
        assert!(classes.contains("backdrop-blur-md"));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_div() {
        let classes = resolve_flowbite_classes("div", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
    }

    #[test]
    fn resolve_flowbite_header_glassmorphism_light() {
        let classes = resolve_flowbite_classes("header", None, ThemeVariant::Glassmorphism, false);
        assert!(classes.contains("backdrop-blur-md"));
        assert!(classes.contains("border-b"));
        assert!(classes.contains("px-6"));
        assert!(classes.contains("py-4"));
    }

    #[test]
    fn resolve_flowbite_sidebar_default_light() {
        let classes =
            resolve_flowbite_classes("sidebar", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("w-64"));
        assert!(classes.contains("h-full"));
        assert!(classes.contains("overflow-y-auto"));
    }

    #[test]
    fn resolve_flowbite_sidebar_collapsed() {
        let classes = resolve_flowbite_classes(
            "sidebar",
            Some("collapsed"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("w-16"));
    }

    #[test]
    fn resolve_flowbite_spacer() {
        let classes =
            resolve_flowbite_classes("spacer", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("flex-1"));
    }

    #[test]
    fn resolve_flowbite_divider_light() {
        let classes =
            resolve_flowbite_classes("divider", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("border-t"));
        assert!(classes.contains(""));
        assert!(classes.contains("my-4"));
    }

    #[test]
    fn resolve_flowbite_divider_dark() {
        let classes =
            resolve_flowbite_classes("divider", None, ThemeVariant::MaterialDesign3, true);
        assert!(classes.contains("border-t"));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_tooltip_light() {
        let classes =
            resolve_flowbite_classes("tooltip", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains("text-xs"));
    }

    #[test]
    fn resolve_flowbite_modal_default_light() {
        let classes = resolve_flowbite_classes("modal", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
        assert!(classes.contains("p-6"));
    }

    #[test]
    fn resolve_flowbite_modal_alert_glassmorphism() {
        let classes =
            resolve_flowbite_classes("modal", Some("alert"), ThemeVariant::Glassmorphism, true);
        assert!(classes.contains("backdrop-blur-xl"));
        assert!(classes.contains(""));
        assert!(classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_avatar_circle() {
        let classes = resolve_flowbite_classes(
            "avatar",
            Some("circle"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("rounded-full"));
    }

    #[test]
    fn resolve_flowbite_avatar_square() {
        let classes = resolve_flowbite_classes(
            "avatar",
            Some("square"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("rounded-none"));
    }

    #[test]
    fn resolve_flowbite_icon_light() {
        let classes = resolve_flowbite_classes("icon", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("w-5"));
        assert!(classes.contains("h-5"));
    }

    #[test]
    fn resolve_flowbite_image() {
        let classes = resolve_flowbite_classes("image", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("w-full"));
        assert!(classes.contains("h-auto"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("object-cover"));
    }

    #[test]
    fn resolve_flowbite_tabs_primary_light() {
        let classes = resolve_flowbite_classes(
            "tabs",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("border-b-2"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("text-sm"));
    }

    #[test]
    fn resolve_flowbite_unknown_component_defaults_to_surface() {
        let classes = resolve_flowbite_classes(
            "unknown-component",
            None,
            ThemeVariant::MaterialDesign3,
            false,
        );
        // Should return surface classes, not empty
        assert!(!classes.is_empty());
        // Should have text color for the theme
        assert!(classes.contains("") || classes.contains(""));
    }

    #[test]
    fn resolve_flowbite_action_button_alias() {
        // action-button should resolve same as button
        let action_classes = resolve_flowbite_classes(
            "action-button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        let button_classes = resolve_flowbite_classes(
            "button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert_eq!(action_classes, button_classes);
    }

    #[test]
    fn resolve_flowbite_action_input_alias() {
        let action_classes = resolve_flowbite_classes(
            "action-input",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        let input_classes = resolve_flowbite_classes(
            "input",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert_eq!(action_classes, input_classes);
    }

    #[test]
    fn resolve_flowbite_none_variant_uses_default() {
        // None variant should behave same as "default"
        let classes =
            resolve_flowbite_classes("button", None, ThemeVariant::MaterialDesign3, false);
        assert!(classes.contains("")); // default primary
        assert!(classes.contains("px-5"));
    }

    #[test]
    fn resolve_flowbite_button_all_themes_produce_non_empty() {
        let themes = [
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
        ];
        for theme in themes {
            for is_dark in [false, true] {
                let classes = resolve_flowbite_classes("button", Some("primary"), theme, is_dark);
                assert!(
                    !classes.is_empty(),
                    "Empty classes for theme {:?}, dark={}",
                    theme,
                    is_dark
                );
            }
        }
    }

    #[test]
    fn resolve_flowbite_card_all_variants_m3_light() {
        let variants = ["elevated", "flat", "outline"];
        for variant in variants {
            let classes = resolve_flowbite_classes(
                "card",
                Some(variant),
                ThemeVariant::MaterialDesign3,
                false,
            );
            assert!(!classes.is_empty(), "Empty for card variant={}", variant);
            assert!(
                classes.contains("rounded"),
                "Missing rounded for card variant={}",
                variant
            );
        }
    }

    #[test]
    fn resolve_flowbite_badge_all_variants_m3_light() {
        let variants = [
            "primary",
            "secondary",
            "success",
            "warning",
            "danger",
            "outline",
        ];
        for variant in variants {
            let classes = resolve_flowbite_classes(
                "badge",
                Some(variant),
                ThemeVariant::MaterialDesign3,
                false,
            );
            assert!(!classes.is_empty(), "Empty for badge variant={}", variant);
            assert!(
                classes.contains("rounded-full"),
                "Missing rounded-full for badge variant={}",
                variant
            );
        }
    }

    // === Button-specific helper tests (internal functions via resolve_flowbite_classes) ===

    #[test]
    fn resolve_flowbite_button_contains_transition_classes() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(classes.contains("transition"));
        assert!(classes.contains("duration-200"));
        assert!(classes.contains("disabled:opacity-50"));
        assert!(classes.contains("disabled:cursor-not-allowed"));
    }

    #[test]
    fn resolve_flowbite_button_includes_padding() {
        let classes = resolve_flowbite_classes(
            "button",
            Some("primary"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        // Button base + transition classes include px-5 py-2.5
        assert!(classes.contains("px-5"));
        assert!(classes.contains("py-2.5"));
    }

    // === Input/Textarea/Select share same base ===

    #[test]
    fn resolve_flowbite_textarea_same_as_input() {
        let input_classes = resolve_flowbite_classes(
            "input",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        let textarea_classes = resolve_flowbite_classes(
            "textarea",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert_eq!(textarea_classes, input_classes);
    }

    #[test]
    fn resolve_flowbite_select_same_as_input() {
        let input_classes = resolve_flowbite_classes(
            "input",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        let select_classes = resolve_flowbite_classes(
            "select",
            Some("outline"),
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert_eq!(input_classes, select_classes);
    }
}
