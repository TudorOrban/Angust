use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::Element, styles::{AlignItems, Margin}};


pub fn allocate_space_to_children_row_flex(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();

    let children_max_height_index = find_max_child_height_index(container);
    let max_height_child = &container.children[children_max_height_index];
    let children_max_height = max_height_child.get_effective_size().height;
    let max_height_child_margin = max_height_child.get_styles().margin.unwrap_or_default();

    let mut current_position = Position {
        x: allocated_position.x + padding.left.value,
        y: allocated_position.y + padding.top.value,
    };

    for child in &mut container.children {
        let child_effective_size = child.get_effective_size();
        let margin = child.get_styles().margin.unwrap_or_default();

        let child_position = compute_child_position_row(
            current_position, align_items, child_effective_size, children_max_height, max_height_child_margin, margin
        );

        child.allocate_space(child_position, child_effective_size);

        current_position.x += spacing.spacing_x.value + margin.left.value + child_effective_size.width + margin.right.value;
    }
}

fn find_max_child_height_index(container: &Container) -> usize {
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

fn compute_child_position_row(
    current_position: Position,
    align_items: AlignItems, 
    child_effective_size: Size,
    children_max_height: f32, 
    max_height_child_margin: Margin,
    margin: Margin,
) -> Position {
    let y_offset = get_y_offset_based_on_align_items(
        align_items, child_effective_size, children_max_height, max_height_child_margin, margin
    );

    Position {
        x: margin.left.value + current_position.x,
        y: current_position.y + y_offset,
    }
}

fn get_y_offset_based_on_align_items(
    align_items: AlignItems,
    child_effective_size: Size,
    children_max_height: f32,
    max_height_child_margin: Margin,
    margin: Margin,
) -> f32 {
    match align_items {
        AlignItems::FlexStart => margin.top.value,
        AlignItems::FlexEnd => children_max_height + max_height_child_margin.vertical() - child_effective_size.height - margin.bottom.value,
        AlignItems::Center => (children_max_height - child_effective_size.height) / 2.0 + max_height_child_margin.top.value,
        AlignItems::Stretch | AlignItems::Baseline => margin.top.value, // Simplified; Baseline needs additional logic
    }
}
