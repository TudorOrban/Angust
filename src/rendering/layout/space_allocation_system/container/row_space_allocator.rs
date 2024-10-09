use crate::rendering::elements::{common_types::{Position, Size}, container::Container, element::{Element, ElementType}, styles::{AlignItems, Directions, FlexWrap, JustifyContent, Margin, Overflow, Spacing}};

pub fn allocate_space_to_children_row_flex(container: &mut Container, allocated_position: Position, allocated_size: Size) {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let align_items = container.get_styles().align_items.unwrap_or_default();
    let flex_wrap = container.get_styles().flex_wrap.unwrap_or_default();
    let overflow = container.get_styles().overflow.unwrap_or_default();

    let children_requested_width = precompute_requested_children_width(container);
    let overflow_width = children_requested_width - (allocated_size.width - padding.horizontal());
    if overflow == Overflow::Auto && children_requested_width > allocated_size.width {
        container.scrollbar_state.thumb_scrollbar_width_ratio = (allocated_size.width - padding.horizontal()) / children_requested_width;
        container.scrollbar_state.is_overflowing = Directions { horizontal: true, vertical: false };
    }

    let children_max_height_index = find_max_child_height_index(container);
    let max_height_child = &container.children[children_max_height_index];
    let children_max_height = max_height_child.get_effective_size().height;
    let max_height_child_margin = max_height_child.get_styles().margin.unwrap_or_default();

    let mut current_position = allocated_position;
    current_position.x += padding.left.value;
    current_position.y += padding.top.value;

    // Adjust current position based on the current scroll position
    if container.scrollbar_state.is_overflowing.horizontal {
        current_position.x -= container.scrollbar_state.current_scroll_position.x * overflow_width;
    }

    let mut remaining_allocated_width = allocated_size.width - padding.horizontal();

    for child in &mut container.children {
        if remaining_allocated_width <= 0.0 && flex_wrap == FlexWrap::NoWrap && overflow != Overflow::Visible {
            break;
        }

        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();

        let child_allocated_position = compute_row_child_position(
            current_position, align_items, child_effective_size, children_max_height, max_height_child_margin, child_margin
        );
        let child_allocated_size = determine_allocated_size(
            child_effective_size, remaining_allocated_width, flex_wrap, overflow, child_margin, spacing
        );

        child.allocate_space(child_allocated_position, child_allocated_size);

        let allocated_space = spacing.spacing_x.value + child_margin.left.value + child_allocated_size.width + child_margin.right.value;
        current_position.x += allocated_space;
        remaining_allocated_width -= allocated_space;
    }
}

fn precompute_requested_children_width(container: &Container) -> f32 {
    let spacing = container.get_styles().spacing.unwrap_or_default();

    container.children.iter().fold(0.0, |acc, child| {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_width = child_margin.horizontal() + child_effective_size.width + spacing.spacing_x.value;
        acc + total_child_width
    })
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

fn compute_row_child_position(
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

fn determine_allocated_size(
    child_effective_size: Size,
    remaining_allocated_width: f32,
    flex_wrap: FlexWrap,
    overflow: Overflow,
    child_margin: Margin,
    spacing: Spacing,
) -> Size {
    if flex_wrap != FlexWrap::NoWrap {
        return child_effective_size; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return child_effective_size;
    }

    let needed_width = child_margin.horizontal() + spacing.spacing_x.value + child_effective_size.width;
    if needed_width <= remaining_allocated_width {
        return child_effective_size; // No need to clip
    }

    Size {
        width: (remaining_allocated_width - child_margin.horizontal() - spacing.spacing_x.value).max(0.0),
        height: child_effective_size.height,
    }
}