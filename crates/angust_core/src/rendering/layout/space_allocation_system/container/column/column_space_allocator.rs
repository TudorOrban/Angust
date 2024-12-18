use crate::rendering::{
    elements::{
        common_types::{Position, Size},
        container::Container,
        element::Element,
        styles::{FlexWrap, Overflow},
    }, 
    layout::{
        size_estimation_system::{child_size_estimator, percentage_size_estimator}, 
        space_allocation_system::container::{column::flex_wrap_allocator, utils}
    }
};

use super::{deficit_resolver, position_allocator, size_allocator, surplus_resolver};


/*
 * Function to allocate space to children in a flex column container.
 */
pub fn allocate_space_to_children_column_flex(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = utils::unwrap_container_styles(container);

    // Compute percentage width children effective sizes as allocated_size.width is now known in the second pass
    percentage_size_estimator::estimate_percentage_height_sizes(container, allocated_size.height);

    // Identify and resolve vertical deficits
    let requested_height = child_size_estimator::precompute_requested_children_height(container);
    if requested_height > allocated_size.height && container.get_styles().flex_wrap.unwrap_or_default() != FlexWrap::NoWrap {
        flex_wrap_allocator::allocate_space_to_column_flex_wrap(container, allocated_position, allocated_size);
        return;
    }

    let mut vertical_deficit = requested_height - allocated_size.height;
    let scrollbar_offset = deficit_resolver::resolve_deficits_column(
        container, allocated_size, requested_height, &mut vertical_deficit
    );

    // Identify and resolve horizontal surplus according to justify-content
    let (mut cursor_position, justify_content_spacing) = surplus_resolver::resolve_vertical_space_surplus(container, allocated_position, - vertical_deficit);
    
    if overflow == Overflow::Auto {
        cursor_position.y -= scrollbar_offset;
    }
    
    // Prepare AlignItems y computations
    let all_indices: Vec<usize> = (0..container.children.len()).collect();
    let (children_max_width, max_width_child_margin) = 
        child_size_estimator::get_max_width_child_properties(container, &all_indices);

    for (index, child) in container.children.iter_mut().enumerate() {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();

        let child_allocated_position = position_allocator::determine_allocated_position_column(
            flex_wrap, overflow, align_items, spacing, justify_content_spacing,
            cursor_position, child_effective_size, index,
            children_max_width, max_width_child_margin, child_margin,
        );
        
        let child_allocated_size = size_allocator::determine_allocated_size_column(
            flex_wrap, overflow,
            child_effective_size, allocated_size,
        );

        child.allocate_space(child_allocated_position, child_allocated_size);

        cursor_position.y = child_allocated_position.y + child_allocated_size.height + child_margin.bottom.value;
    }
}
