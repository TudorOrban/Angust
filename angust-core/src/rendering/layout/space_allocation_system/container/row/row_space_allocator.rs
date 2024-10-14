use crate::rendering::elements::{
    common_types::{Position, Size},
    container::Container,
    element::Element,
    styles::{Margin, Overflow},
};

use super::{deficit_resolver, position_allocator, size_allocator};

pub fn allocate_space_to_children_row_flex(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    // Unwrap styles
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();
    let flex_wrap = container.get_styles().flex_wrap.unwrap_or_default();
    let overflow = container.get_styles().overflow.unwrap_or_default();

    // Resolve horizontal space deficits
    let requested_width = size_allocator::precompute_requested_children_width(container);
    let mut horizontal_deficit = requested_width - allocated_size.width;
    let scrollbar_offset = deficit_resolver::attempt_deficit_resolution(container, allocated_size, requested_width, &mut horizontal_deficit);

    // Prepare AlignItems y computations
    let (children_max_height, max_height_child_margin) = get_max_height_child_properties(container);

    // Start allocating space to children
    let mut current_position = allocated_position;
    current_position.x += padding.left.value;
    current_position.y += padding.top.value;

    if overflow == Overflow::Auto {
        current_position.x -= scrollbar_offset;
    }

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();

        let child_allocated_position = position_allocator::determine_allocated_position(
            flex_wrap, overflow, align_items, spacing,
            current_position, child_effective_size,
            children_max_height, max_height_child_margin, child_margin,
        );
        
        let child_allocated_size = size_allocator::determine_allocated_size(
            flex_wrap, overflow,
            child_effective_size, allocated_size,
        );

        child.allocate_space(child_allocated_position, child_allocated_size);

        current_position.x = child_allocated_position.x + child_allocated_size.width + child_margin.right.value;
    }
}


fn get_max_height_child_properties(container: &mut Container) -> (f32, Margin) {
    let children_max_height_index = position_allocator::find_max_child_height_index(container);
    let max_height_child = if children_max_height_index.is_some() {
        Some(&container.children[children_max_height_index.unwrap()])
    } else {
        None
    };
    let mut children_max_height = 0.0;
    let mut max_height_child_margin = Default::default();

    if max_height_child.is_some() {
        children_max_height = max_height_child.unwrap().get_effective_size().height;
        max_height_child_margin = max_height_child.unwrap().get_styles().margin.unwrap_or_default();
    }

    (children_max_height, max_height_child_margin)
}