use crate::rendering::elements::{common_types::Position, container::Container, element::Element, styles::JustifyContent};


pub fn resolve_horizontal_space_surplus(container: &Container, initial_position: Position, horizontal_surplus: f32) -> (Position, f32) {
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


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::elements::{common_types::Position, styles::{Dimension, Padding, Styles, Unit, JustifyContent}};

    #[test]
    fn test_resolve_horizontal_space_surplus() {
        let cases = [
            (JustifyContent::FlexStart, 0.0, 60.0, 0.0),
            (JustifyContent::FlexEnd, 50.0, 110.0, 0.0),
            (JustifyContent::Center, 25.0, 72.5, 0.0),
            (JustifyContent::SpaceBetween, 100.0, 60.0, 25.0), // TODO: Add SpaceAround case
        ];

        for (justify_content, surplus, expected_start_x, expected_spacing) in cases {
            let mut container = Container::new();
            container.set_styles(Styles {
                padding: Some(Padding {
                    left: Dimension { value: 10.0, unit: Unit::Px },
                    top: Dimension { value: 5.0, unit: Unit::Px },
                    ..Default::default()
                }),
                justify_content: Some(justify_content),
                ..Default::default()
            });

            let num_children = 
                if justify_content == JustifyContent::SpaceBetween || justify_content == JustifyContent::SpaceAround  { 5 } else { 1 };

            for _ in 0..num_children {
                let child = Container::new(); 
                container.add_child(Box::new(child));
            }

            let initial_position = Position { x: 50.0, y: 50.0 };
            let (position, spacing) = resolve_horizontal_space_surplus(&container, initial_position, surplus);

            assert_eq!(position.x, expected_start_x, "Start X failed for {:?}", justify_content);
            assert_eq!(spacing, expected_spacing, "Spacing failed for {:?}", justify_content);
        }
    }
}
