use crate::rendering::elements::{container::Container, element::Element, styles::Margin};

/*
 * Util module for precomputing dimensions of a parent container (to prepare flex-wrap, align-items, overflow, etc.).
 */
// Flex Row
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

pub fn get_max_height_child_properties(container: &Container, indices: &[usize]) -> (f32, Margin) {
    let mut children_max_height = 0.0;
    let mut max_height_child_margin = Margin::default();

    for &index in indices {
        let child = &container.children[index];
        let child_size = child.get_effective_size();
        if child_size.height > children_max_height {
            children_max_height = child_size.height;
            max_height_child_margin = child.get_styles().margin.unwrap_or_default();
        }
    }

    (children_max_height, max_height_child_margin)
}

// Flex Column
pub fn precompute_requested_children_height(container: &Container) -> f32 {
    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();

    container.children.iter().fold(0.0, |acc, child| {
        let child_effective_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_height = spacing.spacing_y.value + child_margin.vertical() + child_effective_size.height;
        acc + total_child_height
    }) + padding.vertical()
}

pub fn get_max_width_child_properties(container: &Container, indices: &[usize]) -> (f32, Margin) {
    let mut children_max_width = 0.0;
    let mut max_width_child_margin = Margin::default();

    for &index in indices {
        let child = &container.children[index];
        let child_size = child.get_effective_size();
        if child_size.width > children_max_width {
            children_max_width = child_size.width;
            max_width_child_margin = child.get_styles().margin.unwrap_or_default();
        }
    }

    (children_max_width, max_width_child_margin)
}



// Tests
#[cfg(test)]
mod tests {
    use crate::rendering::elements::{common_types::OptionalSize, styles::{Dimension, Padding, Spacing, Styles, Unit}};

    use super::*; 

    #[test]
    fn test_precompute_requested_children_width() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding { left: Dimension { value: 10.0, unit: Unit::Px }, right: Dimension { value: 10.0, unit: Unit::Px }, ..Default::default() }),
            spacing: Some(Spacing { spacing_x: Dimension { value: 5.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });

        let mut first_child = Container::new();
        first_child.set_requested_size(OptionalSize { width: Some(Dimension { value: 100.0, unit: Unit::Px }), height: Some(Dimension { value: 200.0, unit: Unit::Px }) });
        first_child.set_styles(Styles {
            margin: Some(Margin{ left: Dimension { value: 5.0, unit: Unit::Px }, right: Dimension { value: 5.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });

        let mut second_child = Container::new();
        second_child.set_requested_size(OptionalSize { width: Some(Dimension { value: 50.0, unit: Unit::Px }), height: Some(Dimension { value: 100.0, unit: Unit::Px }) });
        second_child.set_styles(Styles {
            margin: Some(Margin{ left: Dimension { value: 3.0, unit: Unit::Px }, right: Dimension { value: 3.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });
        
        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));
        
        // Act
        let total_width = precompute_requested_children_width(&container);

        // Assert
        let expected_total_width = 10.0 + 5.0 + (5.0 + 5.0 + 100.0) + 5.0 + (3.0 + 3.0 + 50.0) + 10.0; // Padding + Spacing + First Child Total Width + Spacing + Second Child Total Width + Padding
        assert_eq!(total_width, expected_total_width);
    }

    #[test]
    fn test_get_max_height_child_properties() {
        // Arrange
        let mut container = Container::new();

        let mut first_child = Container::new();
        first_child.set_requested_size(OptionalSize { width: Some(Dimension { value: 100.0, unit: Unit::Px }), height: Some(Dimension { value: 200.0, unit: Unit::Px }) });
        first_child.set_styles(Styles {
            margin: Some(Margin{ top: Dimension { value: 100.0, unit: Unit::Px }, bottom: Dimension { value: 100.0, unit: Unit::Px }, ..Default::default() }),
            ..Default::default()
        });

        let mut second_child = Container::new();
        second_child.set_requested_size(OptionalSize { width: Some(Dimension { value: 100.0, unit: Unit::Px }), height: Some(Dimension { value: 100.0, unit: Unit::Px }) });
        
        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));
        
        // Act
        let (max_height, max_height_child_margin) = get_max_height_child_properties(&container, &[0, 1]);

        // Assert
        assert_eq!(max_height, 200.0);
        assert_eq!(max_height_child_margin.top.value, 100.0);
        assert_eq!(max_height_child_margin.bottom.value, 100.0);
    }
}