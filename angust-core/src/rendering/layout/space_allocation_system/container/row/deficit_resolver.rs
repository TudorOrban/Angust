use crate::rendering::{
    elements::{
        common_types::{OptionalSize, Size}, 
        container::Container, 
        element::Element, 
        styles::{Dimension, Directions, Overflow, Unit, WhiteSpace}
    }, 
    layout::size_estimation_system::child_size_estimator, 
};

/*
 * Function to resolve horizontal space deficits in a flex row container,
 * by applying flex shrink, shrinking text wrappers, and handling overflow.
 */
pub fn resolve_deficits_row(
    container: &mut Container,
    allocated_size: Size,
    requested_width: f32,
    deficit: &mut f32,
) -> f32 {
    if *deficit <= 0.0 {
        return 0.0;
    }

    let effective_horizontal_space = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();
    
    apply_flex_shrink_width(container, deficit);

    let mut new_requested_width = child_size_estimator::precompute_requested_children_width(container);
    let updated_deficit = requested_width - new_requested_width;
    *deficit = updated_deficit.max(0.0);

    if *deficit > 0.0 {
        handle_overflow(container, effective_horizontal_space, deficit, &mut new_requested_width);
    }

    let current_scroll_position_x = container.scrollbar_state.current_scroll_position.x;
    let overflow_width = new_requested_width - allocated_size.width;
    let scrollbar_offset = overflow_width * current_scroll_position_x;

    scrollbar_offset
}

/*
 * Function to apply flex shrink to children to resolve horizontal space deficits.
 */
fn apply_flex_shrink_width(container: &mut Container, deficit: &mut f32) {
    let total_flex_shrink: f32 = container.children.iter()
        .map(|child| child.get_styles().flex_shrink.unwrap_or(0.0) * child.get_effective_size().width)
        .sum();

    if total_flex_shrink <= 0.0 || *deficit <= 0.0 {
        return;
    }

    let total_shrinkage_needed = *deficit;
    for child in &mut container.children {
        let initial_width = child.get_effective_size().width;
        let flex_shrink_factor = child.get_styles().flex_shrink.unwrap_or(0.0);
        let shrink_amount = (flex_shrink_factor * initial_width / total_flex_shrink) * total_shrinkage_needed;
        let new_width = (initial_width - shrink_amount).max(0.0);
        
        if child.get_requested_size().width.is_some() {
            child.set_requested_size(OptionalSize { width: Some(Dimension { value: new_width, unit: Unit::Px }), height: child.get_requested_size().height });
        } else {
            child.set_natural_size(Size { width: new_width, height: child.get_effective_size().height });
        }

        // Update the deficit by the actual amount the width was reduced
        let actual_shrink_amount = initial_width - new_width;
        *deficit -= actual_shrink_amount;
    }
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
            *new_requested_width = child_size_estimator::precompute_requested_children_width(container);
            
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

/*
 * Function to shrink text containers from their natural one-line sizes to resolve horizontal space deficits.
 */
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