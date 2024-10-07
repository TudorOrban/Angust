use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::FlexDirection};


pub fn allocate_space_to_children(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    let mut current_position = allocated_position;

    current_position.x += container.get_styles().padding.unwrap_or_default().left.value;
    current_position.y += container.get_styles().padding.unwrap_or_default().top.value;

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();

        current_position.x += child.get_styles().margin.unwrap_or_default().left.value;
        current_position.y += child.get_styles().margin.unwrap_or_default().top.value;

        child.allocate_space(current_position, child_effective_size);

        match flex_direction {
            FlexDirection::Row => {
                current_position.x += child_effective_size.width;
            },
            FlexDirection::Column => {
                current_position.y += child_effective_size.height;
            },
        }
        
        current_position.x += child.get_styles().margin.unwrap_or_default().right.value;
        current_position.y += child.get_styles().margin.unwrap_or_default().bottom.value;
    }
}