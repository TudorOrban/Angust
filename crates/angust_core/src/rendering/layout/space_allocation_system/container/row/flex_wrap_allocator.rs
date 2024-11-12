use crate::rendering::{
    elements::{common_types::{Position, Size}, container::Container, element::Element}, 
    layout::{size_estimation_system::child_size_estimator, space_allocation_system::container::utils}, 
};

use super::{position_allocator, size_allocator};


pub fn allocate_space_to_row_flex_wrap(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = utils::unwrap_container_styles(container);

    let children_lines = group_children_into_lines(container, allocated_size);

    let mut cursor_position = allocated_position;
    cursor_position.x += container.get_styles().padding.unwrap_or_default().left.value;
    cursor_position.y += container.get_styles().padding.unwrap_or_default().top.value;

    for line in children_lines {
        let (line_max_height, line_max_height_child_margin) = 
            child_size_estimator::get_max_height_child_properties(container, &line.children_indices);

        for &child_index in &line.children_indices {
            let child = &mut container.children[child_index];
            let child_effective_size = child.get_effective_size();
            let child_margin = child.get_styles().margin.unwrap_or_default();

            let child_allocated_position = Position {
                x: cursor_position.x + child_margin.left.value,
                y: position_allocator::compute_child_y_position(
                    align_items, cursor_position, child_effective_size, line_max_height, line_max_height_child_margin, child_margin
                )
            };

            let child_allocated_size = size_allocator::determine_allocated_size_row(
                flex_wrap, overflow, child_effective_size, allocated_size
            );

            child.allocate_space(child_allocated_position, child_allocated_size);

            cursor_position.x += child_allocated_size.width + child_margin.right.value + spacing.spacing_x.value;
        }

        cursor_position.y += line_max_height + line_max_height_child_margin.bottom.value + spacing.spacing_y.value;
        cursor_position.x = allocated_position.x + container.get_styles().padding.unwrap_or_default().left.value;
    }
}

/*
 * Group children into lines based on the available width.
 */
fn group_children_into_lines(container: &Container, allocated_size: Size) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = Line {
        children_indices: Vec::new(),
        total_width: 0.0,
        max_height: 0.0,
    };
    let available_width = allocated_size.width - container.get_styles().padding.unwrap_or_default().horizontal();

    for (index, child) in container.children.iter().enumerate() {
        let child_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_width = child_size.width + child_margin.horizontal();
        
        let space_needed = if current_line.children_indices.is_empty() {
            child_margin.left.value
        } else {
            container.get_styles().spacing.unwrap_or_default().spacing_x.value + child_margin.left.value
        };

        if current_line.total_width + total_child_width + space_needed > available_width && !current_line.children_indices.is_empty() {
            lines.push(current_line);
            current_line = Line {
                children_indices: Vec::new(),
                total_width: 0.0,
                max_height: 0.0,
            };
        }

        // Add the child to the current line
        current_line.children_indices.push(index);
        current_line.total_width += total_child_width + (if current_line.children_indices.len() > 1 { container.get_styles().spacing.unwrap_or_default().spacing_x.value } else { 0.0 });  // Add space only after the first child
        current_line.max_height = current_line.max_height.max(child_size.height);
    }

    if !current_line.children_indices.is_empty() {
        lines.push(current_line);
    }

    lines
}

struct Line {
    children_indices: Vec<usize>,  // Store indices to the children
    total_width: f32,
    max_height: f32,
}


// Tests
#[cfg(test)]
mod tests {
    use crate::rendering::elements::{common_types::OptionalSize, styles::{Dimension, Margin, Padding, Spacing, Styles, Unit}};

    use super::*; 
    
    #[test]
    fn test_allocate_space_to_row_flex_wrap() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding {
                left: Dimension { value: 10.0, unit: Unit::Px },
                top: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut child = Container::new();
        child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 100.0, unit: Unit::Px }),
            height: Some(Dimension { value: 50.0, unit: Unit::Px })
        });
        child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 5.0, unit: Unit::Px },
                right: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        container.add_child(Box::new(child));

        let allocated_position = Position { x: 0.0, y: 0.0 };
        let allocated_size = Size { width: 300.0, height: 100.0 };
        
        // Act
        allocate_space_to_row_flex_wrap(&mut container, allocated_position, allocated_size);

        // Assert
        let allocated_child = &container.children[0];
        assert_eq!(allocated_child.get_position().x, 15.0); // 10 padding + 5 margin
        assert_eq!(allocated_child.get_position().y, 5.0);  // 5 padding
        assert_eq!(allocated_child.get_size().width, 100.0);
        assert_eq!(allocated_child.get_size().height, 50.0);
    }

    #[test]
    fn test_group_children_into_lines() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding {
                left: Dimension { value: 10.0, unit: Unit::Px },
                right: Dimension { value: 10.0, unit: Unit::Px },
                ..Default::default()
            }),
            spacing: Some(Spacing {
                spacing_x: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut first_child = Container::new();
        first_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 100.0, unit: Unit::Px }),
            height: Some(Dimension { value: 20.0, unit: Unit::Px })
        });
        first_child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 5.0, unit: Unit::Px },
                right: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut second_child = Container::new();
        second_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 150.0, unit: Unit::Px }),
            height: Some(Dimension { value: 25.0, unit: Unit::Px })
        });
        second_child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 3.0, unit: Unit::Px },
                right: Dimension { value: 3.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut third_child = Container::new();
        third_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 200.0, unit: Unit::Px }),
            height: Some(Dimension { value: 30.0, unit: Unit::Px })
        });
        third_child.set_styles(Styles {
            margin: Some(Margin {
                left: Dimension { value: 2.0, unit: Unit::Px },
                right: Dimension { value: 2.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));
        container.add_child(Box::new(third_child));

        let allocated_size = Size { width: 300.0, height: 100.0 }; // Constraint the width to 300.0 to force wrapping.

        // Act
        let lines = group_children_into_lines(&container, allocated_size);

        // Assert
        assert_eq!(lines.len(), 2);  // Expect two lines due to width constraints.
        assert_eq!(lines[0].children_indices, vec![0, 1]);  // First and second child fit on the first line.
        assert_eq!(lines[1].children_indices, vec![2]);  // Third child wraps to the next line.
        assert!(lines[0].total_width > 0.0 && lines[1].total_width > 0.0);
        assert!(lines[0].max_height == 25.0 && lines[1].max_height == 30.0);  // Check max height of each line.
    }
}
