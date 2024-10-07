use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::{AlignItems, Margin, Spacing}};


pub fn allocate_space_to_children_row_flex(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let max_height = get_max_child_height(container) - padding.top.value - padding.bottom.value;
    let align_items = container.get_styles().align_items.unwrap_or_default();

    let mut current_position = Position {
        x: allocated_position.x + padding.left.value,
        y: allocated_position.y + padding.top.value,
    };

    for (index, child) in container.children.iter_mut().enumerate() {
        let child_effective_size = child.get_effective_size();
        let margin = child.get_styles().margin.unwrap_or_default();

        let child_position = compute_child_position_row(
            child_effective_size, margin, spacing, align_items, max_height, current_position, index
        );
        if index > 0 {
            current_position.x += spacing.spacing_x.value;
        }
        
        child.allocate_space(child_position, child_effective_size);

        current_position.x += margin.left.value + child_effective_size.width + margin.right.value;
    }
}

fn get_max_child_height(container: &Container) -> f32 {
    let padding = container.get_styles().padding.unwrap_or_default();
    let mut max_child_height: f32 = 0.0;

    for child in &container.children {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();

        let total_child_height = margin.top.value + child_effective_size.height + margin.bottom.value;

        max_child_height = f32::max(max_child_height, total_child_height);
    }

    max_child_height + padding.top.value + padding.bottom.value
}

fn compute_child_position_row(
    child_effective_size: Size,
    margin: Margin,
    spacing: Spacing,
    align_items: AlignItems, 
    max_height: f32, 
    current_position: Position,
    index: usize
) -> Position {
    let x_offset = if index > 0 { spacing.spacing_x.value } else { 0.0 };
    let y_offset = get_y_offset_based_on_align_items(
        align_items, max_height, child_effective_size, margin
    );

    Position {
        x: current_position.x + x_offset,
        y: current_position.y + y_offset,
    }
}

fn get_y_offset_based_on_align_items(
    align_items: AlignItems,
    max_height: f32,
    child_effective_size: Size,
    margin: Margin,
) -> f32 {
    let offset = match align_items {
        AlignItems::FlexStart => margin.top.value,
        AlignItems::FlexEnd => max_height - child_effective_size.height - margin.bottom.value,
        AlignItems::Center => (max_height - child_effective_size.height) / 2.0 + margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => margin.top.value, // Simplified; Baseline needs additional logic
    };

    offset
}
