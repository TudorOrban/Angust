use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::{Directions, Overflow}};

use super::size_allocator;



pub fn attempt_deficit_resolution(
    container: &mut Container,
    allocated_size: Size,
) -> f32 {
    let overflow = container.get_styles().overflow.unwrap_or_default();
    
    let effective_horizontal_space = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();
    let requested_width = size_allocator::precompute_requested_children_width(container);
    
    if requested_width > allocated_size.width {
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
                shrink_text_wrapper_children(container);
                return 0.0;
            }
        }
    }

    let current_scroll_position_x = container.scrollbar_state.current_scroll_position.x;

    let overflow_width = requested_width - allocated_size.width;
    let scrollbar_offset = overflow_width * current_scroll_position_x;

    scrollbar_offset
}

fn shrink_text_wrapper_children(container: &mut Container) {
    let text_wrapper_count: usize = container.children.iter()
        .filter(|child| child.is_text_wrapper())
        .count();

    for child in &mut container.children {
        if !child.is_text_wrapper() {
            continue;
        }

        let child_effective_size = child.get_effective_size();
        
    }
}