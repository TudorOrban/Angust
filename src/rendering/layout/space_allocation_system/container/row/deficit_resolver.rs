use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::{Directions, Overflow}};

use super::size_allocator;



pub fn attempt_deficit_resolution(
    container: &mut Container,
    allocated_size: Size,
) -> f32 {
    // let container_starting_x = allocated_position.x + padding.left.value;
    // let container_ending_x = allocated_position.x + allocated_size.width - padding.right.value;
    let overflow = container.get_styles().overflow.unwrap_or_default();
    
    let effective_horizontal_space = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();
    let requested_width = size_allocator::precompute_requested_children_width(container);
    let deficit = requested_width - allocated_size.width;

    if deficit > 0.0 {
        match overflow {
            Overflow::Auto | Overflow::Scroll => {
                container.scrollbar_state.thumb_scrollbar_width_ratio =
                    effective_horizontal_space / requested_width;
                container.scrollbar_state.is_overflowing = Directions {
                    horizontal: true,
                    vertical: false,
                };
            },
            Overflow::Hidden => {
                return requested_width - allocated_size.width;
            },
            Overflow::Visible => {
                shrink_text_wrapper_children(container, deficit);
                return 0.0;
            }
        }
    }

    let current_scroll_position_x = container.scrollbar_state.current_scroll_position.x;

    let overflow_width = requested_width - allocated_size.width;
    let scrollbar_offset = overflow_width * current_scroll_position_x;

    scrollbar_offset
}

fn shrink_text_wrapper_children(
    container: &mut Container,
    deficit: f32,
) {
    let text_wrapper_count: usize = container.children.iter()
        .filter(|child| child.is_text_wrapper())
        .count();
    if text_wrapper_count == 0 {
        return;
    }

    let min_width_per_text_wrapper = 100.0;
    let mut total_reducible_width = 0.0;

    for child in &mut container.children {
        if !child.is_text_wrapper() {
            continue;
        }

        let current_width = child.get_effective_size().width;
        if current_width > min_width_per_text_wrapper {
            total_reducible_width += current_width - min_width_per_text_wrapper;
        }
    }

    let reduction_ratio = if total_reducible_width > 0.0 {
        (deficit / total_reducible_width).min(1.0)
    } else {
        0.0
    };

    for child in &mut container.children {
        if !child.is_text_wrapper() {
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
    }
}