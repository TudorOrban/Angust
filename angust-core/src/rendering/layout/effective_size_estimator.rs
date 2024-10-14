use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, element::Element, styles::{Dimension, Margin, Unit}};

// Width computations
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

// - Ignore percentage-width children in the first pass of layout algorithm
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

fn find_scale_factor(container: &Container) -> f32 {
    let total_percentage: f32 = container.children.iter()
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
        .sum();

    if total_percentage > 100.0 {
        100.0 / total_percentage
    } else {
        1.0
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

// Height computations
pub fn get_max_height_child_properties(container: &mut Container) -> (f32, Margin) {
    let children_max_height_index = find_max_child_height_index(container);
    let max_height_child = if children_max_height_index.is_some() {
        Some(&container.children[children_max_height_index.unwrap()])
    } else {
        None
    };
    let mut children_max_height = 0.0;
    let mut max_height_child_margin = Default::default();

    if max_height_child.is_some() {
        children_max_height = max_height_child.unwrap().get_effective_size().height;
        max_height_child_margin = max_height_child.unwrap().get_styles().margin.unwrap_or_default();
    }

    (children_max_height, max_height_child_margin)
}

fn find_max_child_height_index(container: &Container) -> Option<usize> {
    let mut max_child_height: f32 = 0.0;
    let mut max_child_height_index: Option<usize> = None;

    for (index, child) in container.children.iter().enumerate() {
        let child_effective_size = child.get_effective_size();

        let total_child_height = child_effective_size.height;

        if total_child_height > max_child_height {
            max_child_height = total_child_height;
            max_child_height_index = Some(index);
        }
    }

    max_child_height_index
}
