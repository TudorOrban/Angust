use crate::rendering::elements::{common_types::{Position, Size}, container::Container, styles::{AlignItems, FlexWrap, Margin, Overflow, Spacing}};


pub fn determine_allocated_position(
    flex_wrap: FlexWrap,
    overflow: Overflow,
    align_items: AlignItems, 
    spacing: Spacing,
    current_position: Position,
    child_effective_size: Size,
    children_max_height: f32, 
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> Position {
    let child_x_position = compute_child_x_position(
        flex_wrap, overflow, spacing, current_position, child_margin
    );
    let child_y_position = compute_child_y_position(
        align_items, current_position, child_effective_size, children_max_height, max_height_child_margin, child_margin
    );

    Position {
        x: child_x_position,
        y: child_y_position,
    }
}

// Horizontal computations
fn compute_child_x_position(
    flex_wrap: FlexWrap,
    overflow: Overflow,
    spacing: Spacing,
    current_position: Position,
    child_margin: Margin,
) -> f32 {
    let new_child_position_x = spacing.spacing_x.value + current_position.x + child_margin.left.value;

    if flex_wrap != FlexWrap::NoWrap {
        return new_child_position_x; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return new_child_position_x; // No need to clip
    }
    
    new_child_position_x
}


// Vertical computations
fn compute_child_y_position(
    align_items: AlignItems,
    current_position: Position,
    child_effective_size: Size,
    children_max_height: f32,
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> f32 {
    let offset = match align_items {
        AlignItems::FlexStart => child_margin.top.value,
        AlignItems::FlexEnd => children_max_height + max_height_child_margin.vertical() - child_effective_size.height - child_margin.bottom.value,
        AlignItems::Center => (children_max_height - child_effective_size.height) / 2.0 + max_height_child_margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => child_margin.top.value, // Simplified; Baseline needs additional logic
    };

    current_position.y + offset
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
