use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::{Directions, Overflow, WhiteSpace}};

use super::size_allocator;



pub fn attempt_deficit_resolution(
    container: &mut Container,
    allocated_size: Size,
) -> f32 {
    let effective_horizontal_space = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();
    let requested_width = size_allocator::precompute_requested_children_width(container);
    
    let mut deficit = requested_width - allocated_size.width;
    let mut new_requested_width = requested_width;

    if deficit > 0.0 {
        handle_overflow(container, effective_horizontal_space, &mut deficit, &mut new_requested_width);
    }

    let current_scroll_position_x = container.scrollbar_state.current_scroll_position.x;
    let overflow_width = new_requested_width - allocated_size.width;
    let scrollbar_offset = overflow_width * current_scroll_position_x;

    scrollbar_offset
}

fn handle_overflow(
    container: &mut Container,
    effective_horizontal_space: f32,
    deficit: &mut f32,
    new_requested_width: &mut f32,
) {
    let overflow = container.get_styles().overflow.unwrap_or_default();

    match overflow {
        Overflow::Auto | Overflow::Scroll => {
            shrink_text_wrapper_children(container, deficit);
            // Recompute requested width after shrinking text wrappers
            *new_requested_width = size_allocator::precompute_requested_children_width(container);
            
            container.scrollbar_state.thumb_scrollbar_width_ratio =
                effective_horizontal_space / new_requested_width.clone();
            container.scrollbar_state.is_overflowing = Directions {
                horizontal: true,
                vertical: false,
            };
        },
        Overflow::Hidden | Overflow::Visible => {},
    }
}

fn shrink_text_wrapper_children(
    container: &mut Container,
    deficit: &mut f32,
) {
    let text_wrapper_count: usize = container.children.iter()
        .filter(|child| is_text_wrapper_shrinkable(child))
        .count();
    if text_wrapper_count == 0 {
        return;
    }

    let min_width_per_text_wrapper = 100.0;
    let reduction_ratio = determine_reduction_ratio(container, deficit.clone(), min_width_per_text_wrapper);    

    for child in &mut container.children {
        if !is_text_wrapper_shrinkable(child) {
            continue;
        }

        let current_size = child.get_effective_size();
        let reducible_amount = current_size.width - min_width_per_text_wrapper;
        let reduction = reducible_amount * reduction_ratio;
        let new_width = current_size.width - reduction;

        child.set_natural_size(Size {
            width: new_width,
            height: current_size.height
        });

        *deficit -= reduction;
    }
}

fn determine_reduction_ratio(
    container: &Container,
    deficit: f32,
    min_width_per_text_wrapper: f32,
) -> f32 {
    let mut total_reducible_width = 0.0;

    for child in container.children.iter() {
        if !child.is_text_wrapper() {
            continue;
        }

        let current_width = child.get_effective_size().width;
        if current_width > min_width_per_text_wrapper {
            total_reducible_width += current_width - min_width_per_text_wrapper;
        }
    }

    if total_reducible_width > 0.0 {
        (deficit / total_reducible_width).min(1.0)
    } else {
        0.0
    }
}

fn is_text_wrapper_shrinkable(
    child: &Box<dyn Element>,
) -> bool {
    child.is_text_wrapper() && child.get_styles().white_space.unwrap_or_default() == WhiteSpace::Normal
}