use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::FlexDirection};


pub fn allocate_space_to_children(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let mut current_position = allocated_position;

    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    for child in &mut container.children {
        let child_width = if let Some(width) = child.get_requested_size().width {
            width.value
        } else {
            child.get_natural_size().width
        };
        let child_height = if let Some(height) = child.get_requested_size().height {
            height.value
        } else {
            child.get_natural_size().height
        };

        child.allocate_space(current_position, Size {
            width: child_width,
            height: child_height,
        });

        match flex_direction {
            FlexDirection::Row => {
                current_position.x += child_width;
            },
            FlexDirection::Column => {
                current_position.y += child_height;
            },
        }
    }
}