//! Tauri to Dioxus schema converter.
//!
//! Converts legacy Tauri schema JSON format to the new Dioxus Schema format.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::data_binding::DataBinding;
use super::page::{CanvasElement, GridPosition, Modal, Page, PageMeta, Schema, Shortcut};

/// Convert a Tauri schema JSON string to a Dioxus Schema.
pub fn convert_tauri_schema(json: &str) -> Result<Schema, String> {
    let tauri_schema: TauriSchema =
        serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(convert_schema(tauri_schema))
}

/// Internal representation of a Tauri schema for conversion.
#[derive(Serialize, Deserialize, Debug)]
struct TauriSchema {
    #[serde(default)]
    app_id: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    pages: Vec<TauriPage>,
    #[serde(default)]
    shortcuts: Vec<TauriShortcut>,
    #[serde(default)]
    modals: Vec<TauriModal>,
    #[serde(default)]
    components: Vec<TauriComponent>,
    #[serde(default)]
    routes: Vec<TauriRoute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriPage {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    component: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    children: Vec<TauriCanvasElement>,
    #[serde(default)]
    elements: Vec<TauriCanvasElement>,
    #[serde(default)]
    meta: Option<TauriPageMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriShortcut {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    keys: Option<String>,
    #[serde(default)]
    action: Option<String>,
    #[serde(default)]
    handler: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriModal {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    components: Vec<TauriCanvasElement>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriComponent {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    component_type: Option<String>,
    #[serde(default)]
    properties: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriRoute {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TauriPageMeta {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    breadcrumbs: Option<Vec<String>>,
    #[serde(default)]
    description: Option<String>,
}

/// Tauri canvas element representation.
#[derive(Serialize, Deserialize, Debug)]
struct TauriCanvasElement {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    component_id: Option<String>,
    #[serde(default)]
    component: Option<String>,
    #[serde(default)]
    #[serde(alias = "componentId")]
    component_id_alias: Option<String>,
    #[serde(default)]
    #[serde(alias = "type")]
    type_field: Option<String>,
    #[serde(default)]
    variant: Option<String>,
    #[serde(default)]
    props: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    properties: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    children: Vec<TauriCanvasElement>,
    #[serde(default)]
    elements: Vec<TauriCanvasElement>,
    #[serde(default)]
    data_context: Option<String>,
    #[serde(default)]
    binding: Option<String>,
    #[serde(default)]
    grid_column: Option<i32>,
    #[serde(default)]
    grid_row: Option<i32>,
    #[serde(default)]
    col: Option<i32>,
    #[serde(default)]
    row: Option<i32>,
    #[serde(default)]
    col_span: Option<i32>,
    #[serde(default)]
    row_span: Option<i32>,
    #[serde(default)]
    visible: Option<bool>,
}

fn convert_schema(tauri: TauriSchema) -> Schema {
    Schema {
        app_id: tauri.app_id.unwrap_or_else(|| "app".to_string()),
        version: tauri.version.unwrap_or_else(|| "1.0.0".to_string()),
        pages: tauri.pages.into_iter().map(convert_page).collect(),
        shortcuts: tauri
            .shortcuts
            .into_iter()
            .filter_map(convert_shortcut)
            .collect(),
        modals: tauri.modals.into_iter().map(convert_modal).collect(),
    }
}

fn convert_page(tauri_page: TauriPage) -> Page {
    let route = tauri_page
        .path
        .clone()
        .or_else(|| tauri_page.name.clone())
        .unwrap_or_default();

    let id = tauri_page
        .name
        .clone()
        .or_else(|| tauri_page.path.clone())
        .unwrap_or_else(|| "page".to_string());

    let mut elements = convert_canvas_elements(tauri_page.children);
    elements.extend(convert_canvas_elements(tauri_page.elements));

    Page {
        id,
        title: tauri_page.title.unwrap_or_default(),
        description: tauri_page.description,
        route,
        layout: String::new(),
        elements,
        meta: convert_page_meta(tauri_page.meta),
        sections: HashMap::new(),
        layout_mode: None,
        data_sources: None,
        actions: None,
    }
}

fn convert_page_meta(tauri_meta: Option<TauriPageMeta>) -> PageMeta {
    tauri_meta
        .map(|m| PageMeta {
            title: m.title,
            icon: m.icon,
            breadcrumb: m.breadcrumbs.unwrap_or_default(),
            description: m.description,
        })
        .unwrap_or_default()
}

fn convert_shortcut(tauri_shortcut: TauriShortcut) -> Option<Shortcut> {
    let keys = tauri_shortcut.keys.clone().or(tauri_shortcut.key.clone())?;

    let id = tauri_shortcut
        .id
        .clone()
        .or_else(|| Some(format!("shortcut-{}", keys.replace("+", "-"))))?;

    Some(Shortcut {
        id,
        keys,
        action: tauri_shortcut
            .action
            .or(tauri_shortcut.handler)
            .unwrap_or_default(),
    })
}

fn convert_modal(tauri_modal: TauriModal) -> Modal {
    let id = tauri_modal
        .id
        .clone()
        .or(tauri_modal.name.clone())
        .unwrap_or_else(|| "modal".to_string());

    let elements = convert_canvas_elements(tauri_modal.components);

    Modal {
        id,
        title: tauri_modal.title.unwrap_or_default(),
        elements,
    }
}

fn convert_canvas_elements(tauri_elements: Vec<TauriCanvasElement>) -> Vec<CanvasElement> {
    tauri_elements
        .into_iter()
        .map(convert_canvas_element)
        .collect()
}

fn convert_canvas_element(tauri_elem: TauriCanvasElement) -> CanvasElement {
    let component = tauri_elem
        .component
        .clone()
        .or(tauri_elem.component_id.clone())
        .or(tauri_elem.component_id_alias.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let id = tauri_elem
        .id
        .clone()
        .unwrap_or_else(|| format!("elem-{}", component.replace("-", "_")));

    let type_field = tauri_elem.type_field.clone();

    let mut props = tauri_elem.props.clone().unwrap_or_default();
    if let Some(properties) = tauri_elem.properties {
        for (key, value) in properties {
            props.insert(key, value);
        }
    }

    let data_binding = tauri_elem
        .binding
        .as_ref()
        .or(tauri_elem.data_context.as_ref())
        .map(|b| {
            let parts: Vec<&str> = b.split('.').collect();
            DataBinding {
                store: parts.first().unwrap_or(&"store").to_string(),
                field: parts.get(1).unwrap_or(&"").to_string(),
                transform: None,
                validator: None,
            }
        });

    let column = tauri_elem.col.or(tauri_elem.grid_column).unwrap_or(1);
    let row = tauri_elem.row.or(tauri_elem.grid_row).unwrap_or(1);
    let col_span = tauri_elem.col_span.unwrap_or(1);
    let row_span = tauri_elem.row_span.unwrap_or(1);

    let children = convert_canvas_elements(tauri_elem.children);

    CanvasElement {
        id,
        component,
        grid_position: GridPosition {
            column,
            row,
            col_span,
            row_span,
            col_start: None,
            row_start: None,
        },
        props,
        #[allow(deprecated)]
        classes: String::new(),
        children,
        data_binding,
        type_field,
        visible: tauri_elem.visible.unwrap_or(true),
        layout: None,
        variant: tauri_elem.variant,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_minimal_tauri_schema() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "pages": [
                {
                    "name": "home",
                    "path": "/",
                    "title": "Home"
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.app_id, "test-app");
        assert_eq!(schema.version, "1.0.0");
        assert_eq!(schema.pages.len(), 1);
        assert_eq!(schema.pages[0].id, "home");
        assert_eq!(schema.pages[0].route, "/");
    }

    #[test]
    fn test_convert_tauri_schema_with_elements() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "pages": [
                {
                    "name": "dashboard",
                    "path": "/dashboard",
                    "title": "Dashboard",
                    "children": [
                        {
                            "id": "card-1",
                            "component": "card",
                            "variant": "elevated"
                        }
                    ]
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.pages[0].elements.len(), 1);
        assert_eq!(schema.pages[0].elements[0].id, "card-1");
        assert_eq!(schema.pages[0].elements[0].component, "card");
        assert_eq!(
            schema.pages[0].elements[0].variant.as_deref(),
            Some("elevated")
        );
    }

    #[test]
    fn test_convert_tauri_schema_with_nested_elements() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "pages": [
                {
                    "name": "dashboard",
                    "path": "/dashboard",
                    "title": "Dashboard",
                    "children": [
                        {
                            "id": "container",
                            "component": "container",
                            "children": [
                                {
                                    "id": "child-1",
                                    "component": "card"
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.pages[0].elements.len(), 1);
        assert_eq!(schema.pages[0].elements[0].children.len(), 1);
        assert_eq!(schema.pages[0].elements[0].children[0].id, "child-1");
    }

    #[test]
    fn test_convert_tauri_shortcuts() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "shortcuts": [
                {
                    "keys": "ctrl+s",
                    "action": "save"
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.shortcuts.len(), 1);
        assert_eq!(schema.shortcuts[0].keys, "ctrl+s");
        assert_eq!(schema.shortcuts[0].action, "save");
    }

    #[test]
    fn test_convert_tauri_modals() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "modals": [
                {
                    "id": "confirm",
                    "title": "Confirm Dialog",
                    "components": [
                        {
                            "id": "ok-btn",
                            "component": "button"
                        }
                    ]
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.modals.len(), 1);
        assert_eq!(schema.modals[0].id, "confirm");
        assert_eq!(schema.modals[0].elements.len(), 1);
    }

    #[test]
    fn test_convert_tauri_grid_position() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "pages": [
                {
                    "name": "grid-page",
                    "path": "/grid",
                    "title": "Grid",
                    "children": [
                        {
                            "id": "cell-1",
                            "component": "cell",
                            "grid_column": 2,
                            "grid_row": 3,
                            "col_span": 2,
                            "row_span": 1
                        }
                    ]
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        let elem = &schema.pages[0].elements[0];
        assert_eq!(elem.grid_position.column, 2);
        assert_eq!(elem.grid_position.row, 3);
        assert_eq!(elem.grid_position.col_span, 2);
        assert_eq!(elem.grid_position.row_span, 1);
    }

    #[test]
    fn test_convert_tauri_data_binding() {
        let json = r#"{
            "app_id": "test-app",
            "version": "1.0.0",
            "pages": [
                {
                    "name": "form",
                    "path": "/form",
                    "title": "Form",
                    "children": [
                        {
                            "id": "input-1",
                            "component": "input",
                            "binding": "form.name"
                        }
                    ]
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        let binding = schema.pages[0].elements[0].data_binding.as_ref().unwrap();
        assert_eq!(binding.store, "form");
        assert_eq!(binding.field, "name");
    }

    #[test]
    fn test_invalid_json_returns_error() {
        let json = "not valid json";
        let result = convert_tauri_schema(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse JSON"));
    }

    #[test]
    fn test_missing_fields_use_defaults() {
        let json = r#"{}"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.app_id, "app");
        assert_eq!(schema.version, "1.0.0");
        assert!(schema.pages.is_empty());
        assert!(schema.shortcuts.is_empty());
        assert!(schema.modals.is_empty());
    }

    #[test]
    fn test_convert_page_meta() {
        let json = r#"{
            "pages": [
                {
                    "name": "page",
                    "path": "/page",
                    "title": "My Page",
                    "meta": {
                        "title": "Custom Title",
                        "icon": "home",
                        "breadcrumbs": ["Home", "Page"],
                        "description": "A page description"
                    }
                }
            ]
        }"#;

        let schema = convert_tauri_schema(json).unwrap();
        assert_eq!(schema.pages[0].meta.title.as_deref(), Some("Custom Title"));
        assert_eq!(schema.pages[0].meta.icon.as_deref(), Some("home"));
        assert_eq!(schema.pages[0].meta.breadcrumb, vec!["Home", "Page"]);
        assert_eq!(
            schema.pages[0].meta.description.as_deref(),
            Some("A page description")
        );
    }
}
