use crate::rendering::elements::{container::Container, element::Element, styles::{AlignItems, FlexWrap, Overflow, Spacing}};


pub fn unwrap_container_styles(container: &Container) -> (Spacing, AlignItems, FlexWrap, Overflow) {
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();
    let flex_wrap = container.get_styles().flex_wrap.unwrap_or_default();
    let overflow = container.get_styles().overflow.unwrap_or_default();

    (spacing, align_items, flex_wrap, overflow)
}
