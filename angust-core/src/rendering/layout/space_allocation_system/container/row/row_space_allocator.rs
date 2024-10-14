use crate::rendering::{elements::{
    common_types::{Position, Size},
    container::Container,
    element::Element,
    styles::{AlignItems, FlexWrap, JustifyContent, Margin, Overflow, Spacing},
}, layout::effective_size_estimator};

use super::{deficit_resolver, position_allocator, size_allocator};

pub fn allocate_space_to_children_row_flex(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = unwrap_container_styles(container);

    // Preadjust children sizes according to flex properties, deficit/surplus
    effective_size_estimator::estimate_percentage_width_sizes(container, allocated_size.width);
    let requested_width = size_allocator::precompute_requested_children_width(container);
    
    let mut horizontal_deficit = requested_width - allocated_size.width;
    let scrollbar_offset = deficit_resolver::resolve_deficits(container, allocated_size, requested_width, &mut horizontal_deficit);

    // Prepare AlignItems y computations
    let (children_max_height, max_height_child_margin) = get_max_height_child_properties(container);

    // Start allocating space to children
    let mut cursor_position = determine_initial_cursor_position(
        container, allocated_position, - horizontal_deficit, scrollbar_offset, &overflow
    );
    println!("Final horizontal deficit: {}", horizontal_deficit);
    let num_children = container.children.len() as f32;
    let justify_content_spacing = match container.get_styles().justify_content.unwrap_or_default() {
        JustifyContent::SpaceBetween if num_children > 1.0 => - horizontal_deficit / (num_children - 1.0),
        JustifyContent::SpaceAround if num_children > 0.0 => - horizontal_deficit / (num_children + 1.0),
        _ => 0.0,
    };
    println!("justify_content_spacing: {}", justify_content_spacing);

    for (index, child) in container.children.iter_mut().enumerate() {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();

        let child_allocated_position = position_allocator::determine_allocated_position(
            flex_wrap, overflow, align_items, spacing, justify_content_spacing,
            cursor_position, child_effective_size, index,
            children_max_height, max_height_child_margin, child_margin,
        );
        
        let child_allocated_size = size_allocator::determine_allocated_size(
            flex_wrap, overflow,
            child_effective_size, allocated_size,
        );

        child.allocate_space(child_allocated_position, child_allocated_size);

        cursor_position.x = child_allocated_position.x + child_allocated_size.width + child_margin.right.value;
    }
}

fn unwrap_container_styles(container: &Container) -> (Spacing, AlignItems, FlexWrap, Overflow) {
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();
    let flex_wrap = container.get_styles().flex_wrap.unwrap_or_default();
    let overflow = container.get_styles().overflow.unwrap_or_default();

    (spacing, align_items, flex_wrap, overflow)
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

fn determine_initial_cursor_position(
    container: &Container,
    allocated_position: Position,
    horizontal_surplus: f32,
    scrollbar_offset: f32,
    overflow: &Overflow,
) -> Position {
    let mut cursor_position = apply_justify_content(container, allocated_position, horizontal_surplus);
    
    if *overflow == Overflow::Auto {
        cursor_position.x -= scrollbar_offset;
    }

    cursor_position.clone()
}

fn apply_justify_content(container: &Container, initial_position: Position, horizontal_surplus: f32) -> Position {
    let padding = container.get_styles().padding.unwrap_or_default();
    let num_children = container.children.len() as f32;
    let mut start_x = initial_position.x + padding.left.value;

    match container.get_styles().justify_content.unwrap_or_default() {
        JustifyContent::FlexStart => {}
        JustifyContent::FlexEnd => start_x += horizontal_surplus,
        JustifyContent::Center => start_x += horizontal_surplus / 2.0,
        JustifyContent::SpaceBetween => {}
        JustifyContent::SpaceAround => start_x += horizontal_surplus / (num_children + 1.0),
    }

    Position { x: start_x, y: initial_position.y + padding.top.value }
}