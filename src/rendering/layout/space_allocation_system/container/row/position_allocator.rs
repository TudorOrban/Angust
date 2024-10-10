use crate::rendering::elements::{common_types::{Position, Size}, container::Container, styles::{AlignItems, FlexWrap, Margin, Overflow}};


pub fn determine_allocated_position(
    current_position: Position,
    container_allocated_position: Position,
    current_scroll_position_x: f32, // Between 0.0 and 1.0
    flex_wrap: FlexWrap,
    overflow: Overflow,
    container_starting_x: f32,
    container_ending_x: f32,
    container_width: f32,
    container_requested_width: f32,
    align_items: AlignItems, 
    child_effective_size: Size,
    children_max_height: f32, 
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> Position {
    let child_x_position = child_x_position(
        current_position, container_allocated_position, current_scroll_position_x, flex_wrap, overflow, container_starting_x, container_ending_x, container_width, container_requested_width, child_margin, child_effective_size.width
    );
    let y_offset = get_y_offset(
        align_items, child_effective_size, children_max_height, max_height_child_margin, child_margin
    );

    Position {
        x: child_x_position,
        y: current_position.y + y_offset,
    }
}

fn child_x_position(
    current_position: Position,
    container_allocated_position: Position,
    current_scroll_position_x: f32, // Between 0.0 and 1.0
    flex_wrap: FlexWrap,
    overflow: Overflow,
    container_starting_x: f32,
    container_ending_x: f32,
    container_width: f32,
    container_requested_width: f32,
    child_margin: Margin,
    child_width: f32,
) -> f32 {
    if flex_wrap != FlexWrap::NoWrap {
        return current_position.x + child_margin.left.value; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return current_position.x + child_margin.left.value; // No need to clip
    }
    
    let new_child_x_position = current_position.x.max(container_starting_x);

    new_child_x_position
}

fn get_y_offset(
    align_items: AlignItems,
    child_effective_size: Size,
    children_max_height: f32,
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> f32 {
    match align_items {
        AlignItems::FlexStart => child_margin.top.value,
        AlignItems::FlexEnd => children_max_height + max_height_child_margin.vertical() - child_effective_size.height - child_margin.bottom.value,
        AlignItems::Center => (children_max_height - child_effective_size.height) / 2.0 + max_height_child_margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => child_margin.top.value, // Simplified; Baseline needs additional logic
    }
}

pub fn find_max_child_height_index(container: &Container) -> usize {
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
