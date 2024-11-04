use crate::rendering::elements::{common_types::{Position, Size}, styles::{AlignItems, FlexWrap, Margin, Overflow, Spacing}};


pub fn determine_allocated_position_column(
    flex_wrap: FlexWrap,
    overflow: Overflow,
    align_items: AlignItems, 
    spacing: Spacing,
    justify_content_spacing: f32,

    cursor_position: Position,
    child_effective_size: Size,
    index: usize,

    children_max_height: f32, 
    max_height_child_margin: Margin,
    child_margin: Margin,
) -> Position {
    let child_x_position = compute_child_x_position(
        align_items, 
        cursor_position, child_effective_size, 
        children_max_height, max_height_child_margin, child_margin
    );
    let child_y_position = compute_child_y_position(
        flex_wrap, overflow, spacing, justify_content_spacing, 
        cursor_position, index,
        child_margin
    );

    Position {
        x: child_x_position,
        y: child_y_position,
    }
}

// Horizontal computations
fn compute_child_y_position(
    flex_wrap: FlexWrap,
    overflow: Overflow,
    spacing: Spacing,
    justify_content_spacing: f32,

    cursor_position: Position,
    index: usize,

    child_margin: Margin,
) -> f32 {
    let mut new_child_position_y = cursor_position.y + child_margin.top.value;
    if index > 0 {
        new_child_position_y += spacing.spacing_y.value + justify_content_spacing;
    }

    if flex_wrap != FlexWrap::NoWrap {
        return new_child_position_y; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return new_child_position_y; // No need to clip
    }
    
    new_child_position_y
}


// Vertical computations
pub fn compute_child_x_position(
    align_items: AlignItems,
    cursor_position: Position,
    child_effective_size: Size,
    children_max_width: f32,
    max_width_child_margin: Margin,
    child_margin: Margin,
) -> f32 {
    let offset = match align_items {
        AlignItems::FlexStart => child_margin.left.value,
        AlignItems::FlexEnd => children_max_width + max_width_child_margin.horizontal() - child_effective_size.width - child_margin.right.value,
        AlignItems::Center => (children_max_width - child_effective_size.width) / 2.0 + max_width_child_margin.left.value,
        AlignItems::Stretch | AlignItems::Baseline => child_margin.left.value, // Simplified; Baseline needs additional logic
    };

    cursor_position.y + offset
}