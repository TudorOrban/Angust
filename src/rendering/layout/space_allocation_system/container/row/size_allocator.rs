use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::{FlexWrap, Margin, Overflow, Spacing}};


pub fn precompute_requested_children_width(container: &Container) -> f32 {
    let spacing = container.get_styles().spacing.unwrap_or_default();

    container.children.iter().fold(0.0, |acc, child| {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_width = child_margin.horizontal() + child_effective_size.width + spacing.spacing_x.value;
        acc + total_child_width
    })
}

pub fn determine_allocated_size(
    child_effective_size: Size,
    container_starting_x: f32,
    container_ending_x: f32,
    current_position_x: f32,
    current_scroll_position_x: f32, // Between 0.0 and 1.0
    flex_wrap: FlexWrap,
    overflow: Overflow,
    child_margin: Margin,
    spacing: Spacing,
) -> Size {
    if flex_wrap != FlexWrap::NoWrap {
        return child_effective_size; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return child_effective_size; // No need to clip
    }

    let new_child_ending_x = child_effective_size.width.min(container_ending_x);
    
    Size {
        width: new_child_ending_x - current_position_x,
        height: child_effective_size.height,
    }
}