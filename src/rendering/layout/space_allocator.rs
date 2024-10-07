use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::{AlignItems, FlexDirection, Margin, Padding}};


pub fn allocate_space_to_children(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    if flex_direction == FlexDirection::Row {
        allocate_space_to_children_row_flex(container, allocated_position, allocated_size);
        return;
    } 

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

pub fn allocate_space_to_children_row_flex(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let max_height = get_max_child_height(container) + padding.top.value + padding.bottom.value;
    let align_items = container.get_styles().align_items.unwrap_or_default();

    let mut current_position = Position {
        x: allocated_position.x + padding.left.value,
        y: allocated_position.y + padding.top.value,
    };

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let margin = child.get_styles().margin.unwrap_or_default();

        let child_position = compute_child_position(
            child_effective_size, margin, padding, align_items, max_height, current_position, 
        );
        child.allocate_space(child_position, child_effective_size);

        current_position.x += margin.left.value + child_effective_size.width + margin.right.value;
    }
}

fn compute_child_position(
    child_effective_size: Size,
    margin: Margin,
    padding: Padding,
    align_items: AlignItems, 
    max_height: f32, 
    current_position: Position, 
) -> Position {
    let y_offset = get_y_offset_based_on_align_items(align_items, max_height, child_effective_size, margin, padding);
    Position {
        x: current_position.x + margin.left.value,
        y: current_position.y + y_offset,
    }
}

fn get_max_child_height(container: &Container) -> f32 {
    let mut max_child_height: f32 = 0.0;
    for child in &container.children {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();
        let total_child_height = margin.top.value + child_effective_size.height + margin.bottom.value;
        max_child_height = max_child_height.max(total_child_height);
    }
    max_child_height
}

fn get_y_offset_based_on_align_items(
    align_items: AlignItems,
    max_height: f32,
    child_effective_size: Size,
    margin: Margin,
    padding: Padding,
) -> f32 {
    let available_height = max_height - padding.top.value - padding.bottom.value;

    match align_items {
        AlignItems::FlexStart => margin.top.value + padding.top.value,
        AlignItems::FlexEnd => max_height - child_effective_size.height - margin.bottom.value - padding.bottom.value,
        AlignItems::Center => padding.top.value + (available_height - child_effective_size.height) / 2.0 + margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => margin.top.value + padding.top.value, // Simplified; Baseline would need additional logic
    }
}