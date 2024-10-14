use std::collections::HashMap;

use crate::rendering::{elements::{
    common_types::{Position, Size},
    container::Container,
    element::Element,
    styles::{AlignItems, FlexWrap, Overflow, Spacing},
}, layout::effective_size_estimator};

use super::{deficit_resolver, position_allocator, size_allocator, surplus_resolver};

pub fn allocate_space_to_children_row_flex(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = unwrap_container_styles(container);

    // Compute percentage width children effective sizes as allocated_size.width is now known in the second pass
    effective_size_estimator::estimate_percentage_width_sizes(container, allocated_size.width);

    // Identify and resolve horizontal deficits
    let requested_width = effective_size_estimator::precompute_requested_children_width(container);

    if requested_width <= allocated_size.width && container.get_styles().flex_wrap.unwrap_or_default() != FlexWrap::NoWrap {
        allocate_space_to_row_flex_wrap(container, allocated_position, allocated_size);
        return;
    }

    let mut horizontal_deficit = requested_width - allocated_size.width;
    let scrollbar_offset = deficit_resolver::resolve_deficits(
        container, allocated_size, requested_width, &mut horizontal_deficit
    );

    // Identify and resolve horizontal surplus according to justify-content
    let (mut cursor_position, justify_content_spacing) = surplus_resolver::resolve_space_surplus(container, allocated_position, - horizontal_deficit);
    
    if overflow == Overflow::Auto {
        cursor_position.x -= scrollbar_offset;
    }
    
    // Prepare AlignItems y computations
    let (children_max_height, max_height_child_margin) = effective_size_estimator::get_max_height_child_properties(container);

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

fn allocate_space_to_row_flex_wrap(
    container: &Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = unwrap_container_styles(container);

    let mut cursor_position = allocated_position;
    cursor_position.x += container.get_styles().padding.unwrap_or_default().left.value;
    cursor_position.y += container.get_styles().padding.unwrap_or_default().top.value;

    let mut remaining_width = allocated_size.width;
    
}

fn group_children_into_lines(container: &Container, allocated_size: Size) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = Line {
        children: Vec::new(),
        total_width: 0.0,
        max_height: 0.0,
    };

    for (index, child) in container.children.iter().enumerate() {
        let child_size = child.get_effective_size(); // This should include margin for calculations
        let space_needed = if current_line.children.is_empty() { 0.0 } else { container.get_styles().spacing.unwrap_or_default().spacing_x.value };

        // Check if the current child can fit in the current line or if a new line is needed
        if current_line.total_width + child_size.width + space_needed > allocated_size.width && !current_line.children.is_empty() {
            // Start a new line because the current child does not fit
            lines.push(current_line);
            current_line = Line {
                children: Vec::new(),
                total_width: 0.0,
                max_height: 0.0,
            };
        }

        // Add the current child to the current line
        current_line.children.push(index);
        current_line.total_width += child_size.width + space_needed;
        current_line.max_height = current_line.max_height.max(child_size.height);
    }

    // Don't forget to add the last line if it has any children
    if !current_line.children.is_empty() {
        lines.push(current_line);
    }

    lines
}

pub struct FlexWrapLines {
    pub lines: Vec<Line>,
    pub max_height: f32,
}

struct Line {
    children: Vec<usize>,  // Store indices to the children
    total_width: f32,
    max_height: f32,
}
