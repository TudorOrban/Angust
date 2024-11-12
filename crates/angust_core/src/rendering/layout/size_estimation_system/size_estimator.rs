use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::FlexDirection};

use super::effective_size_estimator;


/*
 * Module used in first pass of layout algorithm;
 * Determines the *natural* and *requested* size of the container 
 * based on the children's *effective* sizes (i.e. requested if specified, natural otherwise).
 */
pub fn estimate_parent_container_sizes(container: &mut Container) {
    let natural_size = estimate_parent_natural_size(container);
    container.set_natural_size(natural_size);
    
    let sizing_policy = container.get_styles().sizing_policy.unwrap_or_default();
    let estimated_requested_size = effective_size_estimator::estimate_requested_size(&sizing_policy.width, &sizing_policy.height);
    container.set_requested_size(estimated_requested_size);
}

fn estimate_parent_natural_size(container: &mut Container) -> Size {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();
    
    match flex_direction {
        FlexDirection::Row => estimate_row_parent_natural_size(container),
        FlexDirection::Column => estimate_column_parent_natural_size(container)
    }
}

fn estimate_row_parent_natural_size(container: &mut Container) -> Size {
    let parent_padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();

    let mut width: f32 = parent_padding.horizontal();
    let mut height: f32 = parent_padding.vertical();

    // Compute natural size of the container based on the children's effective sizes
    for (index, child) in container.children.iter_mut().enumerate() {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();

        if index > 0 {
            width += spacing.spacing_x.value;
        }
        width += margin.left.value + child_effective_size.width + margin.right.value;
        height = height.max(child_effective_size.height + parent_padding.vertical() + margin.vertical());
    }

    Size { width, height }
}

fn estimate_column_parent_natural_size(container: &mut Container) -> Size {
    let parent_padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();

    let mut width: f32 = parent_padding.horizontal();
    let mut height: f32 = parent_padding.vertical();

    // Compute natural size of the container based on the children's effective sizes
    for (index, child) in container.children.iter_mut().enumerate() {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();

        if index > 0 {
            height += spacing.spacing_y.value;
        }
        height += margin.top.value + child_effective_size.height + margin.bottom.value;
        width = width.max(child_effective_size.width + parent_padding.horizontal() + margin.horizontal());
    }

    Size { width, height }
}


pub fn estimate_leaf_container_sizes(container: &mut Container) {
    container.set_natural_size(Size {
        width: container.get_styles().padding.unwrap_or_default().horizontal(),
        height: container.get_styles().padding.unwrap_or_default().vertical(),
    });
    
    if let Some(sizing_policy) = container.get_styles().sizing_policy {
        let estimated_requested_size = effective_size_estimator::estimate_requested_size(&sizing_policy.width, &sizing_policy.height);
        container.set_requested_size(estimated_requested_size);
    }
}


// Tests
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::elements::{common_types::OptionalSize, styles::{Dimension, Margin, Padding, Spacing, Styles, Unit}};

    #[test]
    fn test_estimate_row_parent_natural_size() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding {
                left: Dimension { value: 10.0, unit: Unit::Px }, 
                right: Dimension { value: 10.0, unit: Unit::Px },
                top: Dimension { value: 5.0, unit: Unit::Px },
                bottom: Dimension { value: 5.0, unit: Unit::Px }
            }),
            spacing: Some(Spacing { spacing_x: Dimension { value: 5.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });

        let mut first_child = Container::new();
        first_child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 3.0, unit: Unit::Px },
                right: Dimension { value: 3.0, unit: Unit::Px },
                top: Dimension { value: 2.0, unit: Unit::Px },
                bottom: Dimension { value: 2.0, unit: Unit::Px }
            }),
            ..Default::default()
        });
        first_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 100.0, unit: Unit::Px }),
            height: Some(Dimension { value: 50.0, unit: Unit::Px })
        });

        let mut second_child = Container::new();
        second_child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 2.0, unit: Unit::Px },
                right: Dimension { value: 2.0, unit: Unit::Px },
                top: Dimension { value: 1.0, unit: Unit::Px },
                bottom: Dimension { value: 1.0, unit: Unit::Px }
            }),
            ..Default::default()
        });
        second_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 150.0, unit: Unit::Px }),
            height: Some(Dimension { value: 60.0, unit: Unit::Px })
        });

        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));

        // Act
        let size = estimate_row_parent_natural_size(&mut container);

        // Assert
        let expected_width = 10.0 + 3.0 + 100.0 + 3.0 + 5.0 + 2.0 + 150.0 + 2.0 + 10.0; // Padding + first child margins + first child width + spacing + second child margins + second child width + padding
        let expected_height = 5.0 + 60.0 + 2.0 + 5.0; // Container top padding + max child height (including max vertical margin) + container bottom padding
        assert_eq!(size.width, expected_width);
        assert_eq!(size.height, expected_height);
    }
}
