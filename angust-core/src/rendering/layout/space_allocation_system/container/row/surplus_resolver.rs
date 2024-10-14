use crate::rendering::elements::{common_types::Position, container::Container, element::Element, styles::JustifyContent};


pub fn resolve_space_surplus(container: &Container, initial_position: Position, horizontal_surplus: f32) -> (Position, f32) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let num_children = container.children.len() as f32;
    let mut start_x = initial_position.x + padding.left.value;
    let mut justify_content_spacing = 0.0;

    if horizontal_surplus <= 0.0 {
        return (Position { x: start_x, y: initial_position.y + padding.top.value }, justify_content_spacing);
    }

    match container.get_styles().justify_content.unwrap_or_default() {
        JustifyContent::FlexStart => {
            // No additional start_x modification needed
        },
        JustifyContent::FlexEnd => {
            start_x += horizontal_surplus;
        },
        JustifyContent::Center => {
            start_x += horizontal_surplus / 2.0;
        },
        JustifyContent::SpaceBetween if num_children > 1.0 => {
            justify_content_spacing = horizontal_surplus / (num_children - 1.0);
        },
        JustifyContent::SpaceAround => {
            start_x += horizontal_surplus / (num_children + 1.0);  // Apply initial spacing
            justify_content_spacing = horizontal_surplus / (num_children + 1.0);
        },
        _ => {}
    }

    (Position { x: start_x, y: initial_position.y + padding.top.value }, justify_content_spacing)
}
