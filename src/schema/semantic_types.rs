//! Semantic type registry for SDUI schema.
//! These are the ONLY valid component types in CanvasElement.component.
//! The renderer maps each type to Flowbite Tailwind classes via flowbite_mapping.rs.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticType {
    // Layout
    Div,
    Header,
    Sidebar,
    Spacer,
    Divider,
    Modal,

    // Content
    Text,
    Icon,
    Image,
    Badge,
    Chip,
    Avatar,
    Tooltip,

    // Actions
    Button,
    ActionButton,

    // Forms
    Input,
    Textarea,
    Select,
    ActionInput,
    ActionTextarea,
    ActionSelect,

    // Containers
    Card,
}

impl SemanticType {
    /// Returns true if this type supports children (can render nested elements)
    pub fn supports_children(&self) -> bool {
        matches!(
            self,
            SemanticType::Div
                | SemanticType::Header
                | SemanticType::Sidebar
                | SemanticType::Modal
                | SemanticType::Card
        )
    }

    /// Returns true if this type is an action component (dispatches to ActionBus)
    pub fn is_action_component(&self) -> bool {
        matches!(
            self,
            SemanticType::ActionButton
                | SemanticType::ActionInput
                | SemanticType::ActionTextarea
                | SemanticType::ActionSelect
        )
    }
}

impl std::fmt::Display for SemanticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticType::Div => write!(f, "div"),
            SemanticType::Header => write!(f, "header"),
            SemanticType::Sidebar => write!(f, "sidebar"),
            SemanticType::Spacer => write!(f, "spacer"),
            SemanticType::Divider => write!(f, "divider"),
            SemanticType::Modal => write!(f, "modal"),
            SemanticType::Text => write!(f, "text"),
            SemanticType::Icon => write!(f, "icon"),
            SemanticType::Image => write!(f, "image"),
            SemanticType::Badge => write!(f, "badge"),
            SemanticType::Chip => write!(f, "chip"),
            SemanticType::Avatar => write!(f, "avatar"),
            SemanticType::Tooltip => write!(f, "tooltip"),
            SemanticType::Button => write!(f, "button"),
            SemanticType::ActionButton => write!(f, "action-button"),
            SemanticType::Input => write!(f, "input"),
            SemanticType::Textarea => write!(f, "textarea"),
            SemanticType::Select => write!(f, "select"),
            SemanticType::ActionInput => write!(f, "action-input"),
            SemanticType::ActionTextarea => write!(f, "action-textarea"),
            SemanticType::ActionSelect => write!(f, "action-select"),
            SemanticType::Card => write!(f, "card"),
        }
    }
}

/// All valid semantic type string values (used for fast lookup)
pub fn is_valid_semantic_type(s: &str) -> bool {
    matches!(
        s,
        "div"
            | "header"
            | "sidebar"
            | "spacer"
            | "divider"
            | "modal"
            | "text"
            | "icon"
            | "image"
            | "badge"
            | "chip"
            | "avatar"
            | "tooltip"
            | "button"
            | "action-button"
            | "input"
            | "textarea"
            | "select"
            | "action-input"
            | "action-textarea"
            | "action-select"
            | "card"
    )
}
