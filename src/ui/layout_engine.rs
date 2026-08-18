//! Layout class resolution engine
//!
//! Converts `ElementLayout` schema values into TailwindCSS class strings
//! for flex/grid display, alignment, spacing, and dimensions.

use crate::schema::ElementLayout;

// Re-export for consumers that imported from flowbite_mapping
pub use super::render_component::resolve_flowbite_classes;

// ===== Public API =====

/// Converts an ElementLayout into Tailwind class string.
///
/// Handles display, flex-direction, gap, align-items, justify-content,
/// padding, margin, width, and height.
///
/// Spacing values are converted to Tailwind's 4px grid (value × 4 = Tailwind units).
/// e.g. Spacing::all(4.0) → "p-4"
pub fn resolve_layout_classes(layout: &ElementLayout) -> String {
    let mut classes: Vec<String> = Vec::new();

    match layout.display {
        crate::schema::DisplayType::Flex => classes.push("flex".into()),
        crate::schema::DisplayType::Grid => classes.push("grid".into()),
        crate::schema::DisplayType::Block => classes.push("block".into()),
        crate::schema::DisplayType::Inline => classes.push("inline".into()),
        crate::schema::DisplayType::InlineBlock => classes.push("inline-block".into()),
        crate::schema::DisplayType::None => classes.push("hidden".into()),
    }

    if let Some(dir) = &layout.flex_direction {
        match dir {
            crate::schema::FlexDirection::Row => classes.push("flex-row".into()),
            crate::schema::FlexDirection::RowReverse => classes.push("flex-row-reverse".into()),
            crate::schema::FlexDirection::Column => classes.push("flex-col".into()),
            crate::schema::FlexDirection::ColumnReverse => classes.push("flex-col-reverse".into()),
        }
    }

    if let Some(gap) = layout.gap {
        if gap > 0.0 {
            let gap_unit = (gap * 4.0).round() as i32;
            classes.push(format!("gap-{}", gap_unit));
        }
    }

    if let Some(align) = &layout.align_items {
        match align {
            crate::schema::AlignItems::FlexStart => classes.push("items-start".into()),
            crate::schema::AlignItems::FlexEnd => classes.push("items-end".into()),
            crate::schema::AlignItems::Center => classes.push("items-center".into()),
            crate::schema::AlignItems::Stretch => classes.push("items-stretch".into()),
            crate::schema::AlignItems::Baseline => classes.push("items-baseline".into()),
        }
    }

    if let Some(justify) = &layout.justify_content {
        match justify {
            crate::schema::JustifyContent::FlexStart => classes.push("justify-start".into()),
            crate::schema::JustifyContent::FlexEnd => classes.push("justify-end".into()),
            crate::schema::JustifyContent::Center => classes.push("justify-center".into()),
            crate::schema::JustifyContent::SpaceBetween => classes.push("justify-between".into()),
            crate::schema::JustifyContent::SpaceAround => classes.push("justify-around".into()),
            crate::schema::JustifyContent::SpaceEvenly => classes.push("justify-evenly".into()),
        }
    }

    if let Some(padding) = &layout.padding {
        let top = (padding.top * 4.0).round() as i32;
        let right = (padding.right * 4.0).round() as i32;
        let bottom = (padding.bottom * 4.0).round() as i32;
        let left = (padding.left * 4.0).round() as i32;
        if top == bottom && right == left {
            if top == right {
                classes.push(format!("p-{}", top));
            } else {
                classes.push(format!("py-{} px-{}", top, right));
            }
        } else {
            classes.push(format!("pt-{} pr-{} pb-{} pl-{}", top, right, bottom, left));
        }
    }

    if let Some(margin) = &layout.margin {
        let top = (margin.top * 4.0).round() as i32;
        let right = (margin.right * 4.0).round() as i32;
        let bottom = (margin.bottom * 4.0).round() as i32;
        let left = (margin.left * 4.0).round() as i32;
        if top == bottom && right == left {
            if top == right {
                classes.push(format!("m-{}", top));
            } else {
                classes.push(format!("my-{} mx-{}", top, right));
            }
        } else {
            classes.push(format!("mt-{} mr-{} mb-{} ml-{}", top, right, bottom, left));
        }
    }

    if let Some(width) = &layout.width {
        classes.push(format!("w-[{}px]", width));
    }

    if let Some(height) = &layout.height {
        classes.push(format!("h-[{}px]", height));
    }

    classes.join(" ")
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::semantic_types::SemanticType;
    use crate::schema::AlignItems;
    use crate::schema::DisplayType;
    use crate::schema::FlexDirection;
    use crate::schema::JustifyContent;
    use crate::schema::Spacing;
    use crate::themes::ThemeVariant;
    use proptest::prelude::*;
    use serde_json;

    impl Arbitrary for DisplayType {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::Flex).boxed(),
                Just(Self::Grid).boxed(),
                Just(Self::Block).boxed(),
                Just(Self::Inline).boxed(),
                Just(Self::InlineBlock).boxed(),
                Just(Self::None).boxed(),
            ]
            .boxed()
        }
    }

    impl Arbitrary for FlexDirection {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::Row).boxed(),
                Just(Self::RowReverse).boxed(),
                Just(Self::Column).boxed(),
                Just(Self::ColumnReverse).boxed(),
            ]
            .boxed()
        }
    }

    impl Arbitrary for AlignItems {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::FlexStart).boxed(),
                Just(Self::FlexEnd).boxed(),
                Just(Self::Center).boxed(),
                Just(Self::Stretch).boxed(),
                Just(Self::Baseline).boxed(),
            ]
            .boxed()
        }
    }

    impl Arbitrary for JustifyContent {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::FlexStart).boxed(),
                Just(Self::FlexEnd).boxed(),
                Just(Self::Center).boxed(),
                Just(Self::SpaceBetween).boxed(),
                Just(Self::SpaceAround).boxed(),
                Just(Self::SpaceEvenly).boxed(),
            ]
            .boxed()
        }
    }

    impl Arbitrary for Spacing {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (any::<f32>(), any::<f32>(), any::<f32>(), any::<f32>())
                .prop_map(|(top, right, bottom, left)| Spacing {
                    top,
                    right,
                    bottom,
                    left,
                })
                .boxed()
        }
    }

    impl Arbitrary for ElementLayout {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                any::<DisplayType>(),
                any::<Option<FlexDirection>>(),
                any::<f32>()
                    .prop_filter("gap must be non-negative", |v| *v >= 0.0)
                    .prop_map(Some),
                any::<Option<AlignItems>>(),
                any::<Option<JustifyContent>>(),
                any::<Option<Spacing>>(),
                any::<Option<Spacing>>(),
                any::<Option<String>>(),
                any::<Option<String>>(),
            )
                .prop_map(
                    |(
                        display,
                        flex_direction,
                        gap,
                        align_items,
                        justify_content,
                        padding,
                        margin,
                        width,
                        height,
                    )| {
                        ElementLayout {
                            display,
                            flex_direction,
                            gap,
                            align_items,
                            justify_content,
                            padding,
                            margin,
                            width,
                            height,
                        }
                    },
                )
                .boxed()
        }
    }

    impl Arbitrary for SemanticType {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::Div).boxed(),
                Just(Self::Header).boxed(),
                Just(Self::Sidebar).boxed(),
                Just(Self::Spacer).boxed(),
                Just(Self::Divider).boxed(),
                Just(Self::Modal).boxed(),
                Just(Self::Text).boxed(),
                Just(Self::Icon).boxed(),
                Just(Self::Image).boxed(),
                Just(Self::Badge).boxed(),
                Just(Self::Chip).boxed(),
                Just(Self::Avatar).boxed(),
                Just(Self::Tooltip).boxed(),
                Just(Self::Button).boxed(),
                Just(Self::ActionButton).boxed(),
                Just(Self::Input).boxed(),
                Just(Self::Textarea).boxed(),
                Just(Self::Select).boxed(),
                Just(Self::ActionInput).boxed(),
                Just(Self::ActionTextarea).boxed(),
                Just(Self::ActionSelect).boxed(),
                Just(Self::Card).boxed(),
            ]
            .boxed()
        }
    }

    impl Arbitrary for ThemeVariant {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();
        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::MaterialDesign3).boxed(),
                Just(Self::Glassmorphism).boxed(),
                Just(Self::Claymorphism).boxed(),
                Just(Self::Skeuomorphism).boxed(),
                Just(Self::NeoBrutalism).boxed(),
                Just(Self::Brutalism).boxed(),
                Just(Self::Neumorphism).boxed(),
                Just(Self::Nord).boxed(),
                Just(Self::TokyoNight).boxed(),
                Just(Self::Catppuccin).boxed(),
                Just(Self::RosePine).boxed(),
                Just(Self::Linear).boxed(),
            ]
            .boxed()
        }
    }

    proptest! {
        #[test]
        fn display_type_json_roundtrip(display_type: DisplayType) {
            let json = serde_json::to_string(&display_type).unwrap();
            let decoded: DisplayType = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(display_type, decoded);
        }
    }

    proptest! {
        #[test]
        fn flex_direction_json_roundtrip(fd: FlexDirection) {
            let json = serde_json::to_string(&fd).unwrap();
            let decoded: FlexDirection = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(fd, decoded);
        }
    }

    proptest! {
        #[test]
        fn align_items_json_roundtrip(ai: AlignItems) {
            let json = serde_json::to_string(&ai).unwrap();
            let decoded: AlignItems = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(ai, decoded);
        }
    }

    proptest! {
        #[test]
        fn justify_content_json_roundtrip(jc: JustifyContent) {
            let json = serde_json::to_string(&jc).unwrap();
            let decoded: JustifyContent = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(jc, decoded);
        }
    }

    proptest! {
        #[test]
        fn spacing_json_roundtrip(spacing: Spacing) {
            let json = serde_json::to_string(&spacing).unwrap();
            let decoded: Spacing = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(spacing, decoded);
        }
    }

    proptest! {
        #[test]
        fn element_layout_json_roundtrip(layout: ElementLayout) {
            let json = serde_json::to_string(&layout).unwrap();
            let decoded: ElementLayout = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(layout, decoded);
        }
    }

    proptest! {
        #[test]
        fn theme_variant_json_roundtrip(tv: ThemeVariant) {
            let json = serde_json::to_string(&tv).unwrap();
            let decoded: ThemeVariant = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(tv, decoded);
        }
    }

    proptest! {
        #[test]
        fn semantic_type_json_roundtrip(st: SemanticType) {
            let json = serde_json::to_string(&st).unwrap();
            let decoded: SemanticType = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(st, decoded);
        }
    }

    proptest! {
        #[test]
        fn resolve_layout_classes_never_empty(layout: ElementLayout) {
            let classes = resolve_layout_classes(&layout);
            prop_assert!(!classes.is_empty(), "resolve_layout_classes returned empty for layout {:?}", layout);
        }
    }

    proptest! {
        #[test]
        fn resolve_layout_classes_deterministic(layout: ElementLayout) {
            let first = resolve_layout_classes(&layout);
            let second = resolve_layout_classes(&layout);
            prop_assert_eq!(first, second, "resolve_layout_classes is not deterministic");
        }
    }

    #[test]
    fn resolve_flowbite_classes_known_components() {
        let components = [
            "button",
            "action-button",
            "input",
            "action-input",
            "textarea",
            "action-textarea",
            "select",
            "action-select",
            "text",
            "badge",
            "chip",
            "switch",
            "tabs",
            "avatar",
            "icon",
            "image",
            "card",
            "div",
            "header",
            "sidebar",
            "spacer",
            "divider",
            "tooltip",
            "modal",
        ];
        let variants = [
            Some("primary"),
            Some("secondary"),
            Some("outline"),
            Some("ghost"),
            None,
        ];

        for component in components {
            for variant in variants {
                for theme in [
                    ThemeVariant::MaterialDesign3,
                    ThemeVariant::Glassmorphism,
                    ThemeVariant::Claymorphism,
                    ThemeVariant::NeoBrutalism,
                    ThemeVariant::Neumorphism,
                    ThemeVariant::Nord,
                    ThemeVariant::TokyoNight,
                    ThemeVariant::Catppuccin,
                    ThemeVariant::RosePine,
                    ThemeVariant::Linear,
                ] {
                    for is_dark in [false, true] {
                        let classes = resolve_flowbite_classes(component, variant, theme, is_dark);
                        assert!(
                            !classes.is_empty(),
                            "resolve_flowbite_classes returned empty for component={}, variant={:?}, theme={:?}, is_dark={}",
                            component, variant, theme, is_dark
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn resolve_flowbite_classes_deterministic() {
        let component = "button";
        let variant = Some("primary");
        let theme = ThemeVariant::MaterialDesign3;
        let is_dark = false;

        let first = resolve_flowbite_classes(component, variant, theme, is_dark);
        let second = resolve_flowbite_classes(component, variant, theme, is_dark);
        assert_eq!(first, second);
    }

    #[test]
    fn resolve_flowbite_classes_unknown_component() {
        let classes = resolve_flowbite_classes(
            "unknown-component",
            None,
            ThemeVariant::MaterialDesign3,
            false,
        );
        assert!(!classes.is_empty());
    }

    #[test]
    fn theme_variant_name_never_empty() {
        for theme in [
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
            let name = theme.name();
            assert!(
                !name.is_empty(),
                "ThemeVariant::name() returned empty for {:?}",
                theme
            );
        }
    }
}
