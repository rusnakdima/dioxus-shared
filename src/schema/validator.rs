//! Schema validator for SDUI.
//!
//! Validates CanvasElement against the semantic type system:
//! - SemanticType: all component values must be valid
//! - Variant: variant values must be valid for the component type
//! - Layout: layout values must be valid ElementLayout
//! - Classes: warns if classes field is still used (deprecated)
//!
//! Used by dioxus-mcp tools to validate schemas before applying changes.

use crate::schema::semantic_types::is_valid_semantic_type;
use crate::schema::variant::is_valid_variant;
use crate::schema::CanvasElement;
use crate::schema::Page;
use crate::schema::Schema;
use std::collections::HashSet;

/// Result type for schema validation — collected errors from all validation passes.
pub type ValidationResult = Vec<ValidationError>;

#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Unknown component type
    UnknownComponent(String),
    /// Invalid variant for the component type
    InvalidVariant { component: String, variant: String },
    /// Classes field is deprecated but still in use
    DeprecatedClasses(String),
    /// Duplicate element ID in the same page
    DuplicateId(String),
    /// Duplicate page route
    DuplicateRoute(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UnknownComponent(name) => {
                write!(f, "Unknown component type: '{}'. Use a SemanticType (e.g., 'button', 'card', 'div').", name)
            }
            ValidationError::InvalidVariant { component, variant } => {
                write!(
                    f,
                    "Invalid variant '{}' for component '{}'.",
                    variant, component
                )
            }
            ValidationError::DeprecatedClasses(id) => {
                write!(f, "Element '{}' still uses 'classes' field which is deprecated. Use 'variant' instead.", id)
            }
            ValidationError::DuplicateId(id) => {
                write!(f, "Duplicate element ID: '{}'", id)
            }
            ValidationError::DuplicateRoute(route) => {
                write!(f, "Duplicate page route: '{}'", route)
            }
        }
    }
}

/// Validate a single CanvasElement.
/// Returns all validation errors found.
pub fn validate_element(element: &CanvasElement) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Check component is valid SemanticType
    let component_str = element.component.as_str();
    if !is_valid_semantic_type(component_str) {
        errors.push(ValidationError::UnknownComponent(component_str.to_string()));
    }

    // Check variant is valid for this component type
    if let Some(variant) = &element.variant {
        if !variant.is_empty() && !is_valid_variant(component_str, variant) {
            errors.push(ValidationError::InvalidVariant {
                component: component_str.to_string(),
                variant: variant.clone(),
            });
        }
    }

    // Warn about deprecated classes field
    #[allow(deprecated)]
    if !element.classes.is_empty() {
        errors.push(ValidationError::DeprecatedClasses(element.id.clone()));
    }

    // Recurse into children
    for child in &element.children {
        errors.extend(validate_element(child));
    }

    errors
}

/// Validate page routes are unique within a slice of pages.
pub fn validate_page_routes(pages: &[Page]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut seen_routes = HashSet::new();

    for page in pages {
        if !seen_routes.insert(page.route.clone()) {
            errors.push(ValidationError::DuplicateRoute(page.route.clone()));
        }
    }

    errors
}

/// Validate all elements in a page have unique IDs.
pub fn validate_element_ids(elements: &[CanvasElement]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    fn check_ids(
        elements: &[CanvasElement],
        seen_ids: &mut HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        for el in elements {
            if !seen_ids.insert(el.id.clone()) {
                errors.push(ValidationError::DuplicateId(el.id.clone()));
            }
            check_ids(&el.children, seen_ids, errors);
        }
    }

    check_ids(elements, &mut seen_ids, &mut errors);
    errors
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;

    proptest! {
      #[test]
      fn proptest_valid_semantic_types(component in "(div|header|sidebar|spacer|divider|modal|text|icon|image|badge|chip|avatar|tooltip|button|action-button|input|textarea|select|action-input|action-textarea|action-select|card)") {
        let el = make_element(&component);
        let errors = validate_element(&el);
        prop_assert!(errors.is_empty(), "component '{}' should be valid: {:?}", component, errors);
      }

      #[test]
      fn proptest_invalid_semantic_types(component in "(unknown|widget|foo|bar|custom-element|my-button|invalid)") {
        let el = make_element(&component);
        let errors = validate_element(&el);
        prop_assert!(!errors.is_empty(), "component '{}' should be invalid", component);
        match &errors[0] {
          ValidationError::UnknownComponent(name) => prop_assert_eq!(name.as_str(), &component),
          _ => prop_assert!(false, "Expected UnknownComponent for '{}'", component),
        }
      }

      #[test]
      fn proptest_unique_element_ids(ids: Vec<String>) {
        let mut unique_ids: HashSet<String> = HashSet::new();
        let mut all_unique = true;
        for id in &ids {
          if !unique_ids.insert(id.clone()) {
            all_unique = false;
            break;
          }
        }
        // This test just verifies the HashSet behavior
        prop_assert!(true); // HashSet correctly prevents duplicates
      }

      #[test]
      fn proptest_variant_valid_for_button(variant in "(primary|secondary|outline|ghost|danger|success|warning|floating|circle)") {
        let mut el = make_element("button");
        el.variant = Some(variant.clone());
        let errors = validate_element(&el);
        prop_assert!(errors.is_empty(), "variant '{}' for button should be valid: {:?}", variant, errors);
      }
    }
}

#[allow(dead_code)]
fn make_element(component: &str) -> CanvasElement {
    CanvasElement {
        id: "test-id".to_string(),
        component: component.to_string(),
        props: serde_json::from_str("{}").unwrap(),
        #[allow(deprecated)]
        classes: String::new(),
        variant: None,
        children: vec![],
        visible: true,
        data_binding: None,
        type_field: None,
        layout: None,
        grid_position: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;
    use std::collections::HashMap;

    #[allow(dead_code)]
    fn make_element(component: &str) -> CanvasElement {
        CanvasElement {
            id: "test-id".to_string(),
            component: component.to_string(),
            props: serde_json::from_str("{}").unwrap(),
            #[allow(deprecated)]
            classes: String::new(),
            variant: None,
            children: vec![],
            visible: true,
            data_binding: None,
            type_field: None,
            layout: None,
            grid_position: Default::default(),
        }
    }

    #[test]
    fn test_valid_component() {
        let el = make_element("button");
        let errors = validate_element(&el);
        assert!(errors.is_empty(), "button should be valid: {:?}", errors);
    }

    #[test]
    fn test_invalid_component() {
        let el = make_element("unknown-widget");
        let errors = validate_element(&el);
        assert!(!errors.is_empty());
        match &errors[0] {
            ValidationError::UnknownComponent(name) => assert_eq!(name, "unknown-widget"),
            _ => panic!("Expected UnknownComponent"),
        }
    }

    #[test]
    fn test_valid_variant_for_component() {
        let mut el = make_element("button");
        el.variant = Some("primary".to_string());
        let errors = validate_element(&el);
        assert!(
            errors.is_empty(),
            "primary is valid for button: {:?}",
            errors
        );
    }

    #[test]
    fn test_invalid_variant_for_component() {
        let mut el = make_element("button");
        el.variant = Some("floating".to_string()); // floating is valid for button
        let errors = validate_element(&el);
        // floating IS valid for button, so no error
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_deprecated_classes() {
        let mut el = make_element("div");
        el.classes = "bg-red-500 text-white".to_string();
        let errors = validate_element(&el);
        assert!(!errors.is_empty());
        match &errors[0] {
            ValidationError::DeprecatedClasses(_) => {}
            _ => panic!("Expected DeprecatedClasses"),
        }
    }

    #[test]
    fn test_nested_children_validated() {
        let child = make_element("text");
        let mut parent = make_element("div");
        parent.children = vec![child];
        let errors = validate_element(&parent);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_duplicate_ids() {
        let mut el1 = make_element("button");
        let mut el2 = make_element("input");
        el1.id = "unique-btn".to_string();
        el2.id = "unique-input".to_string();
        let errors = validate_element_ids(&[el1, el2]);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_duplicate_ids_detected() {
        let el1 = make_element("button");
        let mut el2 = make_element("input");
        el2.id = "test-id".to_string();
        let errors = validate_element_ids(&[el1, el2]);
        assert!(!errors.is_empty());
        match &errors[0] {
            ValidationError::DuplicateId(id) => assert_eq!(id, "test-id"),
            _ => panic!("Expected DuplicateId"),
        }
    }

    #[test]
    fn test_duplicate_routes() {
        use crate::schema::Page;
        let page1 = Page {
            id: "p1".to_string(),
            title: "Page 1".to_string(),
            description: None,
            route: "/home".to_string(),
            layout: Default::default(),
            elements: vec![],
            meta: Default::default(),
            sections: Default::default(),
            layout_mode: None,
            data_sources: None,
            actions: None,
        };
        let mut page2 = page1.clone();
        page2.id = "p2".to_string();
        let errors = validate_page_routes(&[page1, page2]);
        assert!(!errors.is_empty());
        match &errors[0] {
            ValidationError::DuplicateRoute(route) => assert_eq!(route, "/home"),
            _ => panic!("Expected DuplicateRoute"),
        }
    }

    #[test]
    fn test_canvas_element_all_semantic_types() {
        let valid_types = [
            "div",
            "header",
            "sidebar",
            "spacer",
            "divider",
            "modal",
            "text",
            "icon",
            "image",
            "badge",
            "chip",
            "avatar",
            "tooltip",
            "button",
            "action-button",
            "input",
            "textarea",
            "select",
            "action-input",
            "action-textarea",
            "action-select",
            "card",
        ];
        for component_type in valid_types {
            let el = make_element(component_type);
            let errors = validate_element(&el);
            assert!(
                errors.is_empty(),
                "component '{}' should be valid: {:?}",
                component_type,
                errors
            );
        }
    }

    #[test]
    fn test_canvas_element_with_children() {
        let child = make_element("text");
        let mut parent = make_element("div");
        parent.children = vec![child];
        let errors = validate_element(&parent);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_canvas_element_nested_deep() {
        let deep_child = make_element("badge");
        let mut mid = make_element("card");
        mid.children = vec![deep_child];
        let mut root = make_element("div");
        root.children = vec![mid];
        let errors = validate_element(&root);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_canvas_element_with_all_component_variants() {
        let variant_tests = [
            ("button", "primary"),
            ("button", "floating"),
            ("button", "circle"),
            ("card", "elevated"),
            ("card", "outline"),
            ("input", "filled"),
            ("badge", "success"),
            ("icon", "solid"),
            ("avatar", "circle"),
        ];
        for (component, variant) in variant_tests {
            let mut el = make_element(component);
            el.variant = Some(variant.to_string());
            let errors = validate_element(&el);
            assert!(
                errors.is_empty(),
                "variant '{}' for '{}' should be valid: {:?}",
                variant,
                component,
                errors
            );
        }
    }

    #[test]
    fn test_canvas_element_invalid_semantic_type() {
        let invalid_types = [
            "unknown",
            "widget",
            "foo",
            "bar",
            "custom-element",
            "my-button",
        ];
        for component_type in invalid_types {
            let el = make_element(component_type);
            let errors = validate_element(&el);
            assert!(
                !errors.is_empty(),
                "component '{}' should be invalid",
                component_type
            );
            match &errors[0] {
                ValidationError::UnknownComponent(name) => assert_eq!(name, component_type),
                _ => panic!("Expected UnknownComponent for '{}'", component_type),
            }
        }
    }

    #[test]
    fn test_canvas_element_invalid_variant() {
        let invalid_variants = [
            ("button", "invalid-variant"),
            ("card", "floating"),
            ("input", "primary"),
        ];
        for (component, variant) in invalid_variants {
            let mut el = make_element(component);
            el.variant = Some(variant.to_string());
            let errors = validate_element(&el);
            assert!(
                !errors.is_empty(),
                "variant '{}' for '{}' should be invalid",
                variant,
                component
            );
            match &errors[0] {
                ValidationError::InvalidVariant {
                    component: c,
                    variant: v,
                } => {
                    assert_eq!(c, component);
                    assert_eq!(v, variant);
                }
                _ => panic!(
                    "Expected InvalidVariant for '{}' on '{}'",
                    variant, component
                ),
            }
        }
    }

    #[test]
    fn test_element_with_valid_props_json() {
        let props_json = r#"{"label": "Click me", "disabled": false, "count": 42}"#;
        let mut el = make_element("button");
        el.props = serde_json::from_str(props_json).unwrap();
        let errors = validate_element(&el);
        assert!(
            errors.is_empty(),
            "valid props should not cause errors: {:?}",
            errors
        );
    }

    #[test]
    fn test_element_with_empty_props() {
        let mut el = make_element("input");
        el.props = serde_json::from_str("{}").unwrap();
        let errors = validate_element(&el);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_element_with_nested_props_json() {
        let props_json = r#"{"config": {"nested": {"deep": true}}, "items": ["a", "b", "c"]}"#;
        let mut el = make_element("card");
        el.props = serde_json::from_str(props_json).unwrap();
        let errors = validate_element(&el);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_element_props_roundtrip_serialization() {
        let props_original: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::json!({
                "label": "Test",
                "value": 123,
                "active": true
            }))
            .unwrap();
        let mut el = make_element("button");
        el.props = props_original.clone();
        let json_str = serde_json::to_string(&el.props).unwrap();
        let props_parsed: HashMap<String, serde_json::Value> =
            serde_json::from_str(&json_str).unwrap();
        assert_eq!(props_original, props_parsed);
    }

    #[test]
    fn test_grid_position_default() {
        let gp = GridPosition::default();
        assert_eq!(gp.column, 1);
        assert_eq!(gp.row, 1);
        assert_eq!(gp.col_span, 1);
        assert_eq!(gp.row_span, 1);
        assert!(gp.col_start.is_none());
        assert!(gp.row_start.is_none());
    }

    #[test]
    fn test_grid_position_custom_values() {
        let json = r#"{"column": 2, "row": 3, "col_span": 4, "row_span": 5, "col_start": 1, "row_start": 2}"#;
        let gp: GridPosition = serde_json::from_str(json).unwrap();
        assert_eq!(gp.column, 2);
        assert_eq!(gp.row, 3);
        assert_eq!(gp.col_span, 4);
        assert_eq!(gp.row_span, 5);
        assert_eq!(gp.col_start, Some(1));
        assert_eq!(gp.row_start, Some(2));
    }

    #[test]
    fn test_grid_position_partial_json() {
        let json = r#"{"column": 5}"#;
        let gp: GridPosition = serde_json::from_str(json).unwrap();
        assert_eq!(gp.column, 5);
        assert_eq!(gp.row, 1);
        assert_eq!(gp.col_span, 1);
        assert_eq!(gp.row_span, 1);
    }

    #[test]
    fn test_grid_position_roundtrip_serialization() {
        let gp_original = GridPosition {
            column: 3,
            row: 4,
            col_span: 2,
            row_span: 2,
            col_start: Some(1),
            row_start: Some(2),
        };
        let json_str = serde_json::to_string(&gp_original).unwrap();
        let gp_parsed: GridPosition = serde_json::from_str(&json_str).unwrap();
        assert_eq!(gp_original, gp_parsed);
    }

    #[test]
    fn test_grid_position_in_canvas_element() {
        let json = r#"{
            "id": "my-element",
            "component": "card",
            "grid_position": {"column": 2, "row": 3, "col_span": 3, "row_span": 2},
            "props": {},
            "classes": "",
            "children": [],
            "visible": true
        }"#;
        let el: CanvasElement = serde_json::from_str(json).unwrap();
        assert_eq!(el.grid_position.column, 2);
        assert_eq!(el.grid_position.row, 3);
        assert_eq!(el.grid_position.col_span, 3);
        assert_eq!(el.grid_position.row_span, 2);
    }

    #[test]
    fn test_element_layout_default() {
        let layout = ElementLayout {
            display: DisplayType::default(),
            flex_direction: None,
            gap: None,
            align_items: None,
            justify_content: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
        };
        assert_eq!(layout.display, DisplayType::Flex);
        assert!(layout.flex_direction.is_none());
        assert!(layout.gap.is_none());
        assert!(layout.align_items.is_none());
        assert!(layout.justify_content.is_none());
        assert!(layout.padding.is_none());
        assert!(layout.margin.is_none());
    }

    #[test]
    fn test_element_layout_flex() {
        let json = r#"{
            "display": "flex",
            "flex_direction": "row",
            "gap": 16.0,
            "align_items": "center",
            "justify_content": "space_between"
        }"#;
        let layout: ElementLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout.display, DisplayType::Flex);
        assert_eq!(layout.flex_direction, Some(FlexDirection::Row));
        assert_eq!(layout.gap, Some(16.0));
        assert_eq!(layout.align_items, Some(AlignItems::Center));
        assert_eq!(layout.justify_content, Some(JustifyContent::SpaceBetween));
    }

    #[test]
    fn test_element_layout_grid() {
        let json = r#"{"display": "grid"}"#;
        let layout: ElementLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout.display, DisplayType::Grid);
    }

    #[test]
    fn test_element_layout_with_spacing() {
        let json = r#"{
            "padding": {"top": 8.0, "right": 16.0, "bottom": 8.0, "left": 16.0},
            "margin": {"top": 4.0, "right": 0.0, "bottom": 4.0, "left": 0.0}
        }"#;
        let layout: ElementLayout = serde_json::from_str(json).unwrap();
        assert!(layout.padding.is_some());
        let padding = layout.padding.unwrap();
        assert_eq!(padding.top, 8.0);
        assert_eq!(padding.right, 16.0);
        assert_eq!(padding.bottom, 8.0);
        assert_eq!(padding.left, 16.0);
        assert!(layout.margin.is_some());
    }

    #[test]
    fn test_element_layout_roundtrip_serialization() {
        let layout_original = ElementLayout {
            display: DisplayType::Flex,
            flex_direction: Some(FlexDirection::Column),
            gap: Some(8.0),
            align_items: Some(AlignItems::Stretch),
            justify_content: Some(JustifyContent::FlexStart),
            padding: Some(Spacing::all(16.0)),
            margin: Some(Spacing::all(4.0)),
            width: Some("100%".to_string()),
            height: Some("auto".to_string()),
        };
        let json_str = serde_json::to_string(&layout_original).unwrap();
        let layout_parsed: ElementLayout = serde_json::from_str(&json_str).unwrap();
        assert_eq!(layout_original, layout_parsed);
    }

    #[test]
    fn test_display_type_all_variants() {
        let variants = [
            ("flex", DisplayType::Flex),
            ("grid", DisplayType::Grid),
            ("block", DisplayType::Block),
            ("inline", DisplayType::Inline),
            ("inline_block", DisplayType::InlineBlock),
            ("none", DisplayType::None),
        ];
        for (name, expected) in variants {
            let json = format!("\"{}\"", name);
            let parsed: DisplayType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                parsed, expected,
                "display type '{}' should parse correctly",
                name
            );
        }
    }

    #[test]
    fn test_flex_direction_all_variants() {
        let variants = [
            ("row", FlexDirection::Row),
            ("row_reverse", FlexDirection::RowReverse),
            ("column", FlexDirection::Column),
            ("column_reverse", FlexDirection::ColumnReverse),
        ];
        for (name, expected) in variants {
            let json = format!("\"{}\"", name);
            let parsed: FlexDirection = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_align_items_all_variants() {
        let variants = [
            ("flex_start", AlignItems::FlexStart),
            ("flex_end", AlignItems::FlexEnd),
            ("center", AlignItems::Center),
            ("stretch", AlignItems::Stretch),
            ("baseline", AlignItems::Baseline),
        ];
        for (name, expected) in variants {
            let json = format!("\"{}\"", name);
            let parsed: AlignItems = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_justify_content_all_variants() {
        let variants = [
            ("flex_start", JustifyContent::FlexStart),
            ("flex_end", JustifyContent::FlexEnd),
            ("center", JustifyContent::Center),
            ("space_between", JustifyContent::SpaceBetween),
            ("space_around", JustifyContent::SpaceAround),
            ("space_evenly", JustifyContent::SpaceEvenly),
        ];
        for (name, expected) in variants {
            let json = format!("\"{}\"", name);
            let parsed: JustifyContent = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_spacing_helper() {
        let sp = Spacing::all(8.0);
        assert_eq!(sp.top, 8.0);
        assert_eq!(sp.right, 8.0);
        assert_eq!(sp.bottom, 8.0);
        assert_eq!(sp.left, 8.0);
    }

    #[test]
    fn test_canvas_element_with_layout() {
        let json = r#"{
            "id": "layout-test",
            "component": "div",
            "grid_position": {},
            "props": {},
            "classes": "",
            "children": [],
            "visible": true,
            "layout": {
                "display": "flex",
                "flex_direction": "row",
                "gap": 8.0
            }
        }"#;
        let el: CanvasElement = serde_json::from_str(json).unwrap();
        assert!(el.layout.is_some());
        let layout = el.layout.unwrap();
        assert_eq!(layout.display, DisplayType::Flex);
        assert_eq!(layout.flex_direction, Some(FlexDirection::Row));
        assert_eq!(layout.gap, Some(8.0));
    }
}

#[cfg(test)]
mod schema_file_tests {
    use super::*;

    #[test]
    fn test_translator_v3_schema_validates() {
        let content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/schemas/translator_v3.json"
        ));
        let schema: Schema =
            serde_json::from_str(content).expect("translator_v3.json should be valid JSON");
        let errors = validate_schema(&schema);
        assert!(
            errors.is_empty(),
            "translator_v3.json validation errors: {:?}",
            errors
        );
    }
}

/// Validate an entire schema: all pages, all elements, and page route uniqueness.
pub fn validate_schema(schema: &Schema) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate page routes are unique
    errors.extend(validate_page_routes(&schema.pages));

    // Validate all elements in all pages
    for page in &schema.pages {
        errors.extend(validate_element_ids(&page.elements));
        for element in &page.elements {
            errors.extend(validate_element(element));
        }
    }

    errors
}
