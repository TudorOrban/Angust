use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, styles::{Dimension, Unit}};


pub fn estimate_effective_size(requested_size: &OptionalSize, natural_size: &Size) -> Size {
    let effective_width = if let Some(width) = requested_size.width {
        width.value
    } else {
        natural_size.width
    };
    let effective_height = if let Some(height) = requested_size.height {
        height.value
    } else {
        natural_size.height
    };

    Size {
        width: effective_width,
        height: effective_height,
    }
}

pub fn estimate_requested_size(width: &Option<Dimension>, height: &Option<Dimension>) -> OptionalSize {
    let mut requested_size = OptionalSize::default();
    if let Some(width) = width {
        if width.unit != Unit::Percent {
            requested_size.width = Some(width.clone());
        } else {
            requested_size.width = Some(Dimension { value: 0.0, unit: Unit::Percent });
        }
    } 
    if let Some(height) = height {
        if height.unit != Unit::Percent {
            requested_size.height = Some(height.clone());
        } else {
            requested_size.height = Some(Dimension { value: 0.0, unit: Unit::Percent });
        }
    }

    requested_size
}

/* 
 * Estimate requested size of children with percentage width in layout algorithm second pass
 */
pub fn estimate_percentage_width_sizes(container: &mut Container, allocated_width: f32) {
    let scale_factor = find_scale_factor(container);

    for child in &mut container.children {
        if child.get_requested_size().width.is_none() {
            continue;
        }
        let dimension = child.get_requested_size().width.unwrap();
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

fn find_scale_factor(container: &Container) -> f32 {
    let total_percentage = find_total_percentage_width(container);
    if total_percentage > 100.0 {
        100.0 / total_percentage
    } else {
        1.0
    }
}

fn find_total_percentage_width(container: &Container) -> f32 {
    container.children.iter()
        .filter_map(|child| {
            if let Some(dimension) = child.get_requested_size().width {
                if dimension.unit == Unit::Percent {
                    Some(dimension.value) // Collect only percentage values
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

