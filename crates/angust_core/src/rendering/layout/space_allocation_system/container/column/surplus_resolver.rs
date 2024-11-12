use crate::rendering::elements::{common_types::Position, container::Container, element::Element, styles::JustifyContent};

pub fn resolve_vertical_space_surplus(container: &Container, initial_position: Position, vertical_surplus: f32) -> (Position, f32) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let num_children = container.children.len() as f32;
    let mut start_y = initial_position.y + padding.top.value;
    let mut justify_content_spacing = 0.0;

    if vertical_surplus <= 0.0 {
        return (Position { x: initial_position.x + padding.left.value, y: start_y }, justify_content_spacing);
    }

    match container.get_styles().justify_content.unwrap_or_default() {
        JustifyContent::FlexStart => {
            // No additional start_x modification needed
        },
        JustifyContent::FlexEnd => {
            start_y += vertical_surplus;
        },
        JustifyContent::Center => {
            start_y += vertical_surplus / 2.0;
        },
        JustifyContent::SpaceBetween if num_children > 1.0 => {
            justify_content_spacing = vertical_surplus / (num_children - 1.0);
        },
        JustifyContent::SpaceAround => {
            start_y += vertical_surplus / (num_children + 1.0);  // Apply initial spacing
            justify_content_spacing = vertical_surplus / (num_children + 1.0);
        },
        _ => {}
    }

    (Position { x: initial_position.x + padding.left.value, y: start_y }, justify_content_spacing)
}
