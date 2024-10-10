use crate::rendering::elements::{common_types::{Position, Size}, container::Container, styles::{AlignItems, Margin}};


pub fn compute_row_child_position(
    current_position: Position,
    align_items: AlignItems, 
    child_effective_size: Size,
    children_max_height: f32, 
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> Position {
    let y_offset = get_y_offset_based_on_align_items(
        align_items, child_effective_size, children_max_height, max_height_child_margin, child_margin
    );

    Position {
        x: child_margin.left.value + current_position.x,
        y: current_position.y + y_offset,
    }
}

fn get_y_offset_based_on_align_items(
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
