use crate::rendering::{
    elements::{common_types::{Position, Size}, container::Container, element::Element}, 
    layout::size_estimation_system::parent_size_estimator, 
};

use super::{position_allocator, size_allocator, utils};


pub fn allocate_space_to_row_flex_wrap(
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
        let (line_max_height, line_max_height_child_margin) = 
            parent_size_estimator::get_max_height_child_properties(container, &line.children_indices);

        for &child_index in &line.children_indices {
            let child = &mut container.children[child_index];
            let child_effective_size = child.get_effective_size();
            let child_margin = child.get_styles().margin.unwrap_or_default();

            let child_allocated_position = Position {
                x: cursor_position.x + child_margin.left.value,
                y: position_allocator::compute_child_y_position(
                    align_items, cursor_position, child_effective_size, line_max_height, line_max_height_child_margin, child_margin
                )
            };

            let child_allocated_size = size_allocator::determine_allocated_size(
                flex_wrap, overflow, child_effective_size, allocated_size
            );

            child.allocate_space(child_allocated_position, child_allocated_size);

            cursor_position.x += child_allocated_size.width + child_margin.right.value + spacing.spacing_x.value;
        }

        cursor_position.y += line_max_height + line_max_height_child_margin.bottom.value + spacing.spacing_y.value;
        cursor_position.x = allocated_position.x + container.get_styles().padding.unwrap_or_default().left.value;
    }
}

/*
 * Group children into lines based on the available width.
 */
fn group_children_into_lines(container: &Container, allocated_size: Size) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = Line {
        children_indices: Vec::new(),
        total_width: 0.0,
        max_height: 0.0,
    };
    let available_width = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();

    for (index, child) in container.children.iter().enumerate() {
        let child_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_width = child_size.width + child_margin.left.value + child_margin.right.value;
        let space_needed = if current_line.children_indices.is_empty() {
            child_margin.left.value
        } else {
            container.get_styles().spacing.unwrap_or_default().spacing_x.value + child_margin.left.value
        };

        if current_line.total_width + total_child_width + space_needed > available_width && !current_line.children_indices.is_empty() {
            lines.push(current_line);
            current_line = Line {
                children_indices: Vec::new(),
                total_width: 0.0,
                max_height: 0.0,
            };
        }

        // Add the child to the current line
        current_line.children_indices.push(index);
        current_line.total_width += total_child_width + (if current_line.children_indices.len() > 1 { container.get_styles().spacing.unwrap_or_default().spacing_x.value } else { 0.0 });  // Add space only after the first child
        current_line.max_height = current_line.max_height.max(child_size.height);
    }

    if !current_line.children_indices.is_empty() {
        lines.push(current_line);
    }

    lines
}

struct Line {
    children_indices: Vec<usize>,  // Store indices to the children
    total_width: f32,
    max_height: f32,
}
