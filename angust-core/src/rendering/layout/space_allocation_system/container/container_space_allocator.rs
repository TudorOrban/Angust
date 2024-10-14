use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::FlexDirection};

use super::{column::column_space_allocator::allocate_space_to_children_column_flex, row::row_space_allocator::allocate_space_to_children_row_flex};


pub fn allocate_space_to_children(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    match flex_direction {
        FlexDirection::Row => allocate_space_to_children_row_flex(container, allocated_position, allocated_size),
        FlexDirection::Column => allocate_space_to_children_column_flex(container, allocated_position, allocated_size)
    }
}