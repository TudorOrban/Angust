use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::{FlexWrap, Overflow}};



pub fn determine_allocated_size(
    flex_wrap: FlexWrap,
    _: Overflow,
    child_effective_size: Size,
    allocated_size: Size,
) -> Size {
    if flex_wrap != FlexWrap::NoWrap {
        return child_effective_size; // To be implemented later
    }

    // if overflow == Overflow::Visible {
    //     return child_effective_size; // No need to clip
    // }

    let clipped_width = child_effective_size.width.min(allocated_size.width);
    
    Size {
        width: clipped_width,
        height: child_effective_size.height,
    }
}

pub fn precompute_requested_children_width(container: &Container) -> f32 {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();

    container.children.iter().fold(0.0, |acc, child| {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_width = spacing.spacing_x.value + child_margin.horizontal() + child_effective_size.width;
        acc + total_child_width
    }) + padding.horizontal()
}