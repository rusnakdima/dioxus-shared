//! Fluent SchemaBuilder API for constructing schemas programmatically.

use std::collections::HashMap;

use super::data_binding::DataBinding;
use super::element_layout::ElementLayout;
use super::handlers::{ActionDef, DataSourceDef};
use super::page::{
    CanvasElement, GridPosition, Modal, Page, PageMeta, PageSection, Schema, Shortcut,
};

/// Fluent builder for constructing Schema objects.
#[derive(Debug, Default)]
pub struct SchemaBuilder {
    app_id: String,
    version: String,
    pages: Vec<Page>,
    shortcuts: Vec<Shortcut>,
    modals: Vec<Modal>,
}

impl SchemaBuilder {
    /// Create a new SchemaBuilder with the given app_id and version.
    pub fn new(app_id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    /// Add a page to the schema.
    pub fn page(mut self, f: impl FnOnce(PageBuilder) -> PageBuilder) -> Self {
        let builder = PageBuilder::default();
        let built = f(builder).build();
        self.pages.push(built);
        self
    }

    /// Add a shortcut to the schema.
    pub fn shortcut(mut self, f: impl FnOnce(ShortcutBuilder) -> ShortcutBuilder) -> Self {
        let builder = ShortcutBuilder::default();
        let built = f(builder).build();
        self.shortcuts.push(built);
        self
    }

    /// Add a modal to the schema.
    pub fn modal(mut self, f: impl FnOnce(ModalBuilder) -> ModalBuilder) -> Self {
        let builder = ModalBuilder::default();
        let built = f(builder).build();
        self.modals.push(built);
        self
    }

    /// Build the final Schema.
    pub fn build(self) -> Schema {
        Schema {
            app_id: self.app_id,
            version: self.version,
            pages: self.pages,
            shortcuts: self.shortcuts,
            modals: self.modals,
        }
    }
}

/// Builder for Page objects.
#[derive(Debug, Default)]
pub struct PageBuilder {
    id: String,
    title: String,
    description: Option<String>,
    route: String,
    layout: String,
    elements: Vec<CanvasElement>,
    meta: PageMeta,
    sections: HashMap<String, PageSection>,
    layout_mode: Option<String>,
    data_sources: Option<Vec<DataSourceDef>>,
    actions: Option<Vec<ActionDef>>,
}

impl PageBuilder {
    /// Set the page ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the page title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the page description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the page route.
    pub fn route(mut self, route: impl Into<String>) -> Self {
        self.route = route.into();
        self
    }

    /// Set the page layout.
    pub fn layout(mut self, layout: impl Into<String>) -> Self {
        self.layout = layout.into();
        self
    }

    /// Set the page layout mode.
    pub fn layout_mode(mut self, layout_mode: impl Into<String>) -> Self {
        self.layout_mode = Some(layout_mode.into());
        self
    }

    /// Add an element to the page.
    pub fn element(mut self, f: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        let builder = ElementBuilder::default();
        let built = f(builder).build();
        self.elements.push(built);
        self
    }

    /// Set the page meta.
    pub fn meta(mut self, meta: PageMeta) -> Self {
        self.meta = meta;
        self
    }

    /// Add a section to the page.
    pub fn section(mut self, name: impl Into<String>, section: PageSection) -> Self {
        self.sections.insert(name.into(), section);
        self
    }

    /// Set data sources for the page.
    pub fn data_sources(mut self, data_sources: Vec<DataSourceDef>) -> Self {
        self.data_sources = Some(data_sources);
        self
    }

    /// Set actions for the page.
    pub fn actions(mut self, actions: Vec<ActionDef>) -> Self {
        self.actions = Some(actions);
        self
    }

    /// Build the Page.
    pub fn build(self) -> Page {
        Page {
            id: self.id,
            title: self.title,
            description: self.description,
            route: self.route,
            layout: self.layout,
            elements: self.elements,
            meta: self.meta,
            sections: self.sections,
            layout_mode: self.layout_mode,
            data_sources: self.data_sources,
            actions: self.actions,
        }
    }
}

/// Builder for Shortcut objects.
#[derive(Debug, Default)]
pub struct ShortcutBuilder {
    id: String,
    keys: String,
    action: String,
}

impl ShortcutBuilder {
    /// Set the shortcut ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the shortcut keys.
    pub fn keys(mut self, keys: impl Into<String>) -> Self {
        self.keys = keys.into();
        self
    }

    /// Set the shortcut action.
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    /// Build the Shortcut.
    pub fn build(self) -> Shortcut {
        Shortcut {
            id: self.id,
            keys: self.keys,
            action: self.action,
        }
    }
}

/// Builder for Modal objects.
#[derive(Debug, Default)]
pub struct ModalBuilder {
    id: String,
    title: String,
    elements: Vec<CanvasElement>,
}

impl ModalBuilder {
    /// Set the modal ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the modal title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Add an element to the modal.
    pub fn element(mut self, f: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        let builder = ElementBuilder::default();
        let built = f(builder).build();
        self.elements.push(built);
        self
    }

    /// Build the Modal.
    pub fn build(self) -> Modal {
        Modal {
            id: self.id,
            title: self.title,
            elements: self.elements,
        }
    }
}

/// Builder for CanvasElement objects.
#[derive(Debug, Default)]
pub struct ElementBuilder {
    id: String,
    component: String,
    grid_position: GridPosition,
    props: HashMap<String, serde_json::Value>,
    classes: String,
    children: Vec<CanvasElement>,
    data_binding: Option<DataBinding>,
    type_field: Option<String>,
    visible: bool,
    layout: Option<ElementLayout>,
    variant: Option<String>,
}

impl ElementBuilder {
    /// Set the element ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the component name.
    pub fn component(mut self, component: impl Into<String>) -> Self {
        self.component = component.into();
        self
    }

    /// Set the grid position.
    pub fn grid_position(mut self, grid_position: GridPosition) -> Self {
        self.grid_position = grid_position;
        self
    }

    /// Set the column for grid position.
    pub fn column(mut self, column: i32) -> Self {
        self.grid_position.column = column;
        self
    }

    /// Set the row for grid position.
    pub fn row(mut self, row: i32) -> Self {
        self.grid_position.row = row;
        self
    }

    /// Set the column span.
    pub fn col_span(mut self, col_span: i32) -> Self {
        self.grid_position.col_span = col_span;
        self
    }

    /// Set the row span.
    pub fn row_span(mut self, row_span: i32) -> Self {
        self.grid_position.row_span = row_span;
        self
    }

    /// Set a prop value.
    pub fn prop(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }

    /// Set multiple props at once.
    pub fn props(mut self, props: HashMap<String, serde_json::Value>) -> Self {
        self.props = props;
        self
    }

    /// Set the element variant.
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Set the element type_field.
    pub fn type_field(mut self, type_field: impl Into<String>) -> Self {
        self.type_field = Some(type_field.into());
        self
    }

    /// Set the element visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set the data binding.
    pub fn data_binding(mut self, data_binding: DataBinding) -> Self {
        self.data_binding = Some(data_binding);
        self
    }

    /// Set the layout.
    pub fn layout(mut self, layout: ElementLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Add a child element.
    pub fn child(mut self, f: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        let builder = ElementBuilder::default();
        let built = f(builder).build();
        self.children.push(built);
        self
    }

    /// Add a child element directly.
    pub fn add_child(mut self, child: CanvasElement) -> Self {
        self.children.push(child);
        self
    }

    /// Build the CanvasElement.
    #[allow(deprecated)]
    pub fn build(self) -> CanvasElement {
        CanvasElement {
            id: self.id,
            component: self.component,
            grid_position: self.grid_position,
            props: self.props,
            classes: self.classes,
            children: self.children,
            data_binding: self.data_binding,
            type_field: self.type_field,
            visible: self.visible,
            layout: self.layout,
            variant: self.variant,
        }
    }
}

/// Helper function to create a page builder closure.
pub fn page(f: impl FnOnce(PageBuilder) -> PageBuilder) -> impl FnOnce(PageBuilder) -> PageBuilder {
    f
}

/// Helper function to create an element builder closure.
pub fn element(
    f: impl FnOnce(ElementBuilder) -> ElementBuilder,
) -> impl FnOnce(ElementBuilder) -> ElementBuilder {
    f
}

/// Helper function to create a shortcut builder closure.
pub fn shortcut(
    f: impl FnOnce(ShortcutBuilder) -> ShortcutBuilder,
) -> impl FnOnce(ShortcutBuilder) -> ShortcutBuilder {
    f
}

/// Helper function to create a modal builder closure.
pub fn modal(
    f: impl FnOnce(ModalBuilder) -> ModalBuilder,
) -> impl FnOnce(ModalBuilder) -> ModalBuilder {
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_builder_basic() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .page(|p| p.id("dashboard").route("/dashboard").title("Dashboard"))
            .build();

        assert_eq!(schema.app_id, "test-app");
        assert_eq!(schema.version, "1.0.0");
        assert_eq!(schema.pages.len(), 1);
        assert_eq!(schema.pages[0].id, "dashboard");
        assert_eq!(schema.pages[0].route, "/dashboard");
    }

    #[test]
    fn test_schema_builder_with_elements() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .page(|p| {
                p.id("dashboard")
                    .route("/dashboard")
                    .title("Dashboard")
                    .element(|e| e.id("card-1").component("card").variant("elevated"))
            })
            .build();

        assert_eq!(schema.pages[0].elements.len(), 1);
        assert_eq!(schema.pages[0].elements[0].component, "card");
        assert_eq!(
            schema.pages[0].elements[0].variant.as_deref(),
            Some("elevated")
        );
    }

    #[test]
    fn test_schema_builder_with_nested_elements() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .page(|p| {
                p.id("dashboard")
                    .route("/dashboard")
                    .title("Dashboard")
                    .element(|e| {
                        e.id("container")
                            .component("container")
                            .child(|c| c.id("child-1").component("card"))
                    })
            })
            .build();

        assert_eq!(schema.pages[0].elements.len(), 1);
        assert_eq!(schema.pages[0].elements[0].children.len(), 1);
        assert_eq!(schema.pages[0].elements[0].children[0].id, "child-1");
    }

    #[test]
    fn test_schema_builder_shortcuts() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .shortcut(|s| s.id("ctrl-s").keys("ctrl+s").action("save"))
            .build();

        assert_eq!(schema.shortcuts.len(), 1);
        assert_eq!(schema.shortcuts[0].keys, "ctrl+s");
    }

    #[test]
    fn test_schema_builder_modals() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .modal(|m| {
                m.id("confirm-dialog")
                    .title("Confirm")
                    .element(|e| e.id("ok-button").component("button"))
            })
            .build();

        assert_eq!(schema.modals.len(), 1);
        assert_eq!(schema.modals[0].id, "confirm-dialog");
        assert_eq!(schema.modals[0].elements.len(), 1);
    }

    #[test]
    fn test_element_builder_props() {
        let schema = SchemaBuilder::new("test-app", "1.0.0")
            .page(|p| {
                p.id("test").route("/test").title("Test").element(|e| {
                    e.id("elem-1")
                        .component("input")
                        .prop("placeholder", "Enter name")
                        .prop("disabled", false)
                })
            })
            .build();

        let props = &schema.pages[0].elements[0].props;
        assert_eq!(
            props.get("placeholder").and_then(|v| v.as_str()),
            Some("Enter name")
        );
        assert_eq!(props.get("disabled").and_then(|v| v.as_bool()), Some(false));
    }
}
