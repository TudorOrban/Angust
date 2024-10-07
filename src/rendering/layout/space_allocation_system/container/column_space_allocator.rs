use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::{AlignItems, Margin}};

pub fn allocate_space_to_children_row_column(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let max_width = get_max_child_width(container) - padding.left.value - padding.right.value;
    let align_items = container.get_styles().align_items.unwrap_or_default();

    let mut current_position = Position {
        x: allocated_position.x + padding.left.value,
        y: allocated_position.y + padding.top.value,
    };

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let margin = child.get_styles().margin.unwrap_or_default();

        let child_position = compute_child_position_column(
            child_effective_size, margin, align_items, max_width, current_position
        );
        child.allocate_space(child_position, child_effective_size);

        current_position.y += margin.top.value + child_effective_size.height + margin.bottom.value;
    }
}

fn compute_child_position_column(
    child_effective_size: Size,
    margin: Margin,
    align_items: AlignItems, 
    max_width: f32, 
    current_position: Position
) -> Position {
    let x_offset = get_x_offset_based_on_align_items(align_items, max_width, child_effective_size, margin);
    Position {
        x: current_position.x + x_offset,
        y: current_position.y,
    }
}

fn get_x_offset_based_on_align_items(
    align_items: AlignItems,
    max_width: f32,
    child_effective_size: Size,
    margin: Margin
) -> f32 {
    match align_items {
        AlignItems::FlexStart => margin.left.value,
        AlignItems::FlexEnd => max_width - child_effective_size.width - margin.right.value,
        AlignItems::Center => (max_width - child_effective_size.width) / 2.0 + margin.left.value,
        AlignItems::Stretch | AlignItems::Baseline => margin.left.value, // Simplified; Baseline needs additional logic
    }
}

fn get_max_child_width(container: &Container) -> f32 {
    container.children.iter().fold(0.0, |acc, child| {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();
        let total_child_width = margin.left.value + child_effective_size.width + margin.right.value;
        f32::max(acc, total_child_width)
    }) + container.get_styles().padding.unwrap_or_default().horizontal()
}
