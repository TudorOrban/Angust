use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::{AlignItems, FlexWrap, JustifyContent, Margin, Overflow}};

use super::row_deficit_resolver::resolve_row_width_deficit;


pub fn allocate_space_to_children_row_flex(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();

    let children_max_height_index = find_max_child_height_index(container);
    let max_height_child = &container.children[children_max_height_index];
    let children_max_height = max_height_child.get_effective_size().height;
    let max_height_child_margin = max_height_child.get_styles().margin.unwrap_or_default();

    let parent_effective_width = precompute_effective_width(container);
    let deficit = allocated_size.width - parent_effective_width;
    if deficit > 0.0 {
        if container.get_styles().flex_wrap.unwrap_or_default() == FlexWrap::NoWrap {
            if container.get_styles().overflow.unwrap_or_default() != Overflow::Visible {
                resolve_row_width_deficit(container, deficit);
            } else {
                // Clip content during loop
            }
        } else {
            // Wrap content during loop
        }
    } else if deficit < 0.0 {
        if container.get_styles().justify_content.unwrap_or_default() == JustifyContent::SpaceBetween {
            // Distribute space between children
        } else {
            // Similarly
        }
    }
    let mut parent_remaining_width = allocated_size.width - padding.left.value - padding.right.value;

    let mut current_position = allocated_position;
    current_position.x += padding.left.value;
    current_position.y += padding.top.value;

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let margin = child.get_styles().margin.unwrap_or_default();

        let child_position = compute_child_position_row(
            current_position, align_items, child_effective_size, children_max_height, max_height_child_margin, margin
        );

        child.allocate_space(child_position, child_effective_size);

        let allocated_space = spacing.spacing_x.value + margin.left.value + child_effective_size.width + margin.right.value;
        current_position.x += allocated_space;
        parent_remaining_width -= allocated_space;
    }
}

fn precompute_effective_width(container: &Container) -> f32 {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();

    let mut total_width = padding.left.value + padding.right.value;
    let mut total_spacing = spacing.spacing_x.value * (container.children.len() as f32 - 1.0);

    for child in &container.children {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();
        total_width += margin.left.value + child_effective_size.width + margin.right.value;
    }

    total_width + total_spacing
}

fn find_max_child_height_index(container: &Container) -> usize {
    let mut max_child_height: f32 = 0.0;
    let mut max_child_height_index: usize = 0;

    for (index, child) in container.children.iter().enumerate() {
        let child_effective_size = child.get_effective_size();

        let total_child_height = child_effective_size.height;

        if total_child_height > max_child_height {
            max_child_height = total_child_height;
            max_child_height_index = index;
        }
    }

    max_child_height_index
}

fn compute_child_position_row(
    current_position: Position,
    align_items: AlignItems, 
    child_effective_size: Size,
    children_max_height: f32, 
    max_height_child_margin: Margin,
    margin: Margin,
) -> Position {
    let y_offset = get_y_offset_based_on_align_items(
        align_items, child_effective_size, children_max_height, max_height_child_margin, margin
    );

    Position {
        x: margin.left.value + current_position.x,
        y: current_position.y + y_offset,
    }
}

fn get_y_offset_based_on_align_items(
    align_items: AlignItems,
    child_effective_size: Size,
    children_max_height: f32,
    max_height_child_margin: Margin,
    margin: Margin,
) -> f32 {
    match align_items {
        AlignItems::FlexStart => margin.top.value,
        AlignItems::FlexEnd => children_max_height + max_height_child_margin.vertical() - child_effective_size.height - margin.bottom.value,
        AlignItems::Center => (children_max_height - child_effective_size.height) / 2.0 + max_height_child_margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => margin.top.value, // Simplified; Baseline needs additional logic
    }
}
