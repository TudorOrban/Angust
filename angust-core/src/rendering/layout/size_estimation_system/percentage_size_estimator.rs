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


// Tests
#[cfg(test)]
mod tests {
    use crate::rendering::elements::{element::Element, styles::{Padding, SizingPolicy, Spacing, Styles}};

    use super::*;

    #[test]
    fn test_estimate_percentage_width_sizes() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding { left: Dimension { value: 10.0, unit: Unit::Px }, right: Dimension { value: 10.0, unit: Unit::Px }, ..Default::default() }),
            spacing: Some(Spacing { spacing_x: Dimension { value: 5.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });

        let mut first_child = Container::new();
        first_child.set_styles(Styles {
            sizing_policy: Some(SizingPolicy { width: Some(Dimension { value: 50.0, unit: Unit::Percent }), ..Default::default() }),
            ..Default::default()
        });
        first_child.set_requested_size(OptionalSize { width: None, height: Some(Dimension { value: 200.0, unit: Unit::Px }) });

        let mut second_child = Container::new();
        second_child.set_styles(Styles {
            sizing_policy: Some(SizingPolicy { width: Some(Dimension { value: 75.0, unit: Unit::Percent }), ..Default::default() }),
            ..Default::default()
        });
        second_child.set_requested_size(OptionalSize { width: None, height: Some(Dimension { value: 100.0, unit: Unit::Px }) });

        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));

        let allocated_width = 1000.0;

        // Act
        estimate_percentage_width_sizes(&mut container, allocated_width);

        // Assert
        let first_child_calculated_width = container.children[0].get_requested_size().width.unwrap().value;
        let second_child_calculated_width = container.children[1].get_requested_size().width.unwrap().value;

        let expected_first_child_width = 50.0 / 100.0 * allocated_width;
        let expected_second_child_width = 75.0 / 100.0 * allocated_width;

        assert_eq!(first_child_calculated_width, expected_first_child_width);
        assert_eq!(second_child_calculated_width, expected_second_child_width);
    }
}
