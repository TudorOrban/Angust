use crate::rendering::elements::{
    common_types::OptionalSize, 
    container::Container, 
    styles::{Dimension, Unit}
};


/* 
 * Module used in layout algorithm second pass;
 * Estimates requested size of children with percentage width
 */
// Flex Row
pub fn estimate_percentage_width_sizes(container: &mut Container, allocated_width: f32) {
    let scale_factor = find_width_scale_factor(container);

    for child in &mut container.children {
        if child.get_styles().sizing_policy.unwrap_or_default().width.is_none() {
            continue;
        }
        let dimension = child.get_styles().sizing_policy.unwrap_or_default().width.unwrap();
        if dimension.unit != Unit::Percent {
            continue;
        }

        let effective_percentage = dimension.value * scale_factor;
        let calculated_width = (effective_percentage / 100.0) * allocated_width;

        child.set_requested_size(OptionalSize {
            width: Some(Dimension {
                value: calculated_width,
                unit: Unit::Px // Convert percentage to absolute pixels
            }),
            height: child.get_requested_size().height
        });
    }
}

fn find_width_scale_factor(container: &Container) -> f32 {
    let total_percentage: f32 = container.children.iter()
        .filter_map(|child| {
            let width_opt = child.get_requested_size().width;
            if width_opt.is_none() {
                return None;
            }
            let width = width_opt.unwrap();

            if width.unit == Unit::Percent {
                Some(width.value) // Collect only percentage values
            } else {
                None
            }
        })
        .sum();

    if total_percentage > 100.0 {
        100.0 / total_percentage
    } else {
        1.0
    }
}

// Flex Column
pub fn estimate_percentage_height_sizes(container: &mut Container, allocated_height: f32) {
    let scale_factor = find_height_scale_factor(container);

    for child in &mut container.children {
        if child.get_styles().sizing_policy.unwrap_or_default().height.is_none() {
            continue;
        }
        let dimension = child.get_styles().sizing_policy.unwrap_or_default().height.unwrap();
        if dimension.unit != Unit::Percent {
            continue;
        }

        let effective_percentage = dimension.value * scale_factor;
        let calculated_height = (effective_percentage / 100.0) * allocated_height;

        child.set_requested_size(OptionalSize {
            width: child.get_requested_size().width,
            height: Some(Dimension {
                value: calculated_height,
                unit: Unit::Px // Convert percentage to absolute pixels
            })
        });
    }
}

fn find_height_scale_factor(container: &Container) -> f32 {
    let total_percentage: f32 = container.children.iter()
        .filter_map(|child| {
            let height_opt = child.get_requested_size().height;
            if height_opt.is_none() {
                return None;
            }
            let height = height_opt.unwrap();

            if height.unit == Unit::Percent {
                Some(height.value) // Collect only percentage values
            } else {
                None
            }
        })
        .sum();

    if total_percentage > 100.0 {
        100.0 / total_percentage
    } else {
        1.0
    }
}