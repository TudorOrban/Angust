use crate::rendering::{
    elements::{common_types::{Position, Size}, container::Container, element::Element}, 
    layout::{size_estimation_system::child_size_estimator, space_allocation_system::container::utils}, 
};

use super::{position_allocator, size_allocator};

#[allow(dead_code)]
pub fn allocate_space_to_column_flex_wrap(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = utils::unwrap_container_styles(container);

    let children_lines = group_children_into_lines(container, allocated_size);

    let mut cursor_position = allocated_position;
    cursor_position.x += container.get_styles().padding.unwrap_or_default().left.value;
    cursor_position.y += container.get_styles().padding.unwrap_or_default().top.value;

    for line in children_lines {
        let (line_max_width, line_max_width_child_margin) = 
            child_size_estimator::get_max_width_child_properties(container, &line.children_indices);

        for &child_index in &line.children_indices {
            let child = &mut container.children[child_index];
            let child_effective_size = child.get_effective_size();
            let child_margin = child.get_styles().margin.unwrap_or_default();

            let child_allocated_position = Position {
                y: cursor_position.y + child_margin.top.value,
                x: position_allocator::compute_child_x_position(
                    align_items, cursor_position, child_effective_size, line_max_width, line_max_width_child_margin, child_margin
                )
            };

            let child_allocated_size = size_allocator::determine_allocated_size_column(
                flex_wrap, overflow, child_effective_size, allocated_size
            );

            child.allocate_space(child_allocated_position, child_allocated_size);

            cursor_position.y += child_allocated_size.height + child_margin.bottom.value + spacing.spacing_y.value;
        }

        cursor_position.x += line_max_width + line_max_width_child_margin.right.value + spacing.spacing_x.value;
        cursor_position.y = allocated_position.y + container.get_styles().padding.unwrap_or_default().top.value;
    }
}

/*
 * Group children into lines based on the available width.
 */
fn group_children_into_lines(container: &Container, allocated_size: Size) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = Line {
        children_indices: Vec::new(),
        total_height: 0.0,
        max_width: 0.0,
    };
    let available_height = allocated_size.height - container.get_styles().padding.unwrap_or_default().vertical();

    for (index, child) in container.children.iter().enumerate() {
        let child_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_height = child_size.height + child_margin.vertical();

        let space_needed = if current_line.children_indices.is_empty() {
            child_margin.top.value
        } else {
            container.get_styles().spacing.unwrap_or_default().spacing_y.value + child_margin.top.value
        };

        if current_line.total_height + total_child_height + space_needed > available_height && !current_line.children_indices.is_empty() {
            lines.push(current_line);
            current_line = Line {
                children_indices: Vec::new(),
                total_height: 0.0,
                max_width: 0.0,
            };
        }

        // Add the child to the current line
        current_line.children_indices.push(index);
        current_line.total_height += total_child_height + (if current_line.children_indices.len() > 1 { container.get_styles().spacing.unwrap_or_default().spacing_y.value } else { 0.0 });  // Add space only after the first child
        current_line.max_width = current_line.max_width.max(child_size.width);
    }

    if !current_line.children_indices.is_empty() {
        lines.push(current_line);
    }

    lines
}

struct Line {
    children_indices: Vec<usize>,  // Store indices to the children
    total_height: f32,
    max_width: f32,
}
