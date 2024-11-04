use crate::rendering::{
    elements::{
        common_types::{OptionalSize, Size}, 
        container::Container, 
        element::Element, 
        styles::{Dimension, Directions, Overflow, Unit}
    }, 
    layout::size_estimation_system::parent_size_estimator, 
};


/*
 * Function to resolve horizontal space deficits in a flex column container,
 * by applying flex shrink, shrinking text wrappers, and handling overflow.
 */
#[allow(dead_code)]
pub fn resolve_deficits_column(
    container: &mut Container,
    allocated_size: Size,
    requested_height: f32,
    deficit: &mut f32,
) -> f32 {
    if *deficit <= 0.0 {
        return 0.0;
    }

    let effective_vertical_space = allocated_size.height - container.get_styles().padding.unwrap_or_default().vertical();
    
    apply_flex_shrink_height(container, deficit);

    let mut new_requested_height = parent_size_estimator::precompute_requested_children_height(container);
    let updated_deficit = requested_height - new_requested_height;
    *deficit = updated_deficit.max(0.0);

    if *deficit > 0.0 {
        handle_overflow(container, effective_vertical_space, deficit, &mut new_requested_height);
    }

    let current_scroll_position_y = container.scrollbar_state.current_scroll_position.y;
    let overflow_height = new_requested_height - allocated_size.height;
    let scrollbar_offset = overflow_height * current_scroll_position_y;

    scrollbar_offset
}

/*
 * Function to apply flex shrink to children to resolve horizontal space deficits.
 */
fn apply_flex_shrink_height(container: &mut Container, deficit: &mut f32) {
    let total_flex_shrink: f32 = container.children.iter()
        .map(|child| child.get_styles().flex_shrink.unwrap_or(0.0) * child.get_effective_size().height)
        .sum();

    if total_flex_shrink <= 0.0 || *deficit <= 0.0 {
        return;
    }

    let total_shrinkage_needed = *deficit;
    for child in &mut container.children {
        let initial_height = child.get_effective_size().height;
        let flex_shrink_factor = child.get_styles().flex_shrink.unwrap_or(0.0);
        let shrink_amount = (flex_shrink_factor * initial_height / total_flex_shrink) * total_shrinkage_needed;
        let new_height = (initial_height - shrink_amount).max(0.0);
        
        if child.get_requested_size().height.is_some() {
            child.set_requested_size(OptionalSize { width: child.get_requested_size().width, height: Some(Dimension { value: new_height, unit: Unit::Px }) });
        } else {
            child.set_natural_size(Size { width: child.get_effective_size().width, height: new_height });
        }

        // Update the deficit by the actual amount the height was reduced
        let actual_shrink_amount = initial_height - new_height;
        *deficit -= actual_shrink_amount;
    }
}

fn handle_overflow(
    container: &mut Container,
    effective_vertical_space: f32,
    _: &mut f32,
    new_requested_height: &mut f32,
) {
    let overflow = container.get_styles().overflow.unwrap_or_default();

    match overflow {
        Overflow::Auto | Overflow::Scroll => {
            // shrink_text_wrapper_children(container, deficit);
            // Recompute requested width after shrinking text wrappers
            // *new_requested_height = parent_size_estimator::precompute_requested_children_height(container);
            
            container.scrollbar_state.thumb_scrollbar_height_ratio =
                effective_vertical_space / new_requested_height.clone();
            container.scrollbar_state.is_overflowing = Directions {
                horizontal: false,
                vertical: true,
            };
        },
        Overflow::Hidden | Overflow::Visible => {},
    }
}

/*
 * Function to shrink text containers from their natural one-line sizes to resolve horizontal space deficits.
 * Currently not in use.
 */
// fn shrink_text_wrapper_children(
//     container: &mut Container,
//     deficit: &mut f32,
// ) {
//     let text_wrapper_count: usize = container.children.iter()
//         .filter(|child| is_text_wrapper_shrinkable(child))
//         .count();
//     if text_wrapper_count == 0 {
//         return;
//     }

//     let min_width_per_text_wrapper = 100.0;
//     let reduction_ratio = determine_reduction_ratio(container, deficit.clone(), min_width_per_text_wrapper);    

//     for child in &mut container.children {
//         if !is_text_wrapper_shrinkable(child) {
//             continue;
//         }

//         let current_size = child.get_effective_size();
//         let reducible_amount = current_size.width - min_width_per_text_wrapper;
//         let reduction = reducible_amount * reduction_ratio;
//         let new_width = current_size.width - reduction;

//         child.set_natural_size(Size {
//             width: new_width,
//             height: current_size.height
//         });

//         *deficit -= reduction;
//     }
// }

// fn determine_reduction_ratio(
//     container: &Container,
//     deficit: f32,
//     min_width_per_text_wrapper: f32,
// ) -> f32 {
//     let mut total_reducible_width = 0.0;

//     for child in container.children.iter() {
//         if !child.is_text_wrapper() {
//             continue;
//         }

//         let current_width = child.get_effective_size().width;
//         if current_width > min_width_per_text_wrapper {
//             total_reducible_width += current_width - min_width_per_text_wrapper;
//         }
//     }

//     if total_reducible_width > 0.0 {
//         (deficit / total_reducible_width).min(1.0)
//     } else {
//         0.0
//     }
// }

// fn is_text_wrapper_shrinkable(
//     child: &Box<dyn Element>,
// ) -> bool {
//     child.is_text_wrapper() && child.get_styles().white_space.unwrap_or_default() == WhiteSpace::Normal
// }