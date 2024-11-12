use crate::rendering::{
    elements::{common_types::{Position, Size}, container::Container, element::Element}, 
    layout::{size_estimation_system::child_size_estimator, space_allocation_system::container::utils}, 
};

use super::{position_allocator, size_allocator};

pub fn allocate_space_to_column_flex_wrap(
    container: &mut Container,
    allocated_position: Position,
    allocated_size: Size,
) {
    let (spacing, align_items, flex_wrap, overflow) = utils::unwrap_container_styles(container);

    let children_lines = group_children_into_column_lines(container, allocated_size);

    let mut cursor_position = allocated_position;
    cursor_position.x += container.get_styles().padding.unwrap_or_default().left.value;
    cursor_position.y += container.get_styles().padding.unwrap_or_default().top.value;

    for line in children_lines {
        let (line_max_width, line_max_width_child_margin) = 
            child_size_estimator::get_max_width_child_properties(container, &line.children_indices);

        for &child_index in &line.children_indices {
            let child = &mut container.children[child_index];
            let child_effective_size = child.get_effective_size();
            let child_margin = child.get_styles().margin.unwrap_or_default();

            let child_allocated_position = Position {
                y: cursor_position.y + child_margin.top.value,
                x: position_allocator::compute_child_x_position(
                    align_items, cursor_position, child_effective_size, line_max_width, line_max_width_child_margin, child_margin
                )
            };

            let child_allocated_size = size_allocator::determine_allocated_size_column(
                flex_wrap, overflow, child_effective_size, allocated_size
            );

            child.allocate_space(child_allocated_position, child_allocated_size);

            cursor_position.y += child_allocated_size.height + child_margin.bottom.value + spacing.spacing_y.value;
        }

        cursor_position.x += line_max_width + line_max_width_child_margin.right.value + spacing.spacing_x.value;
        cursor_position.y = allocated_position.y + container.get_styles().padding.unwrap_or_default().top.value;
    }
}

/*
 * Group children into lines based on the available width.
 */
fn group_children_into_column_lines(container: &Container, allocated_size: Size) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line = Line {
        children_indices: Vec::new(),
        total_height: 0.0,
        max_width: 0.0,
    };
    let available_height = allocated_size.height - container.get_styles().padding.unwrap_or_default().vertical();

    for (index, child) in container.children.iter().enumerate() {
        let child_size = child.get_effective_size();
        let child_margin = child.get_styles().margin.unwrap_or_default();
        let total_child_height = child_size.height + child_margin.vertical();

        let space_needed = if current_line.children_indices.is_empty() {
            child_margin.top.value
        } else {
            container.get_styles().spacing.unwrap_or_default().spacing_y.value + child_margin.top.value
        };

        if current_line.total_height + total_child_height + space_needed > available_height && !current_line.children_indices.is_empty() {
            lines.push(current_line);
            current_line = Line {
                children_indices: Vec::new(),
                total_height: 0.0,
                max_width: 0.0,
            };
        }

        // Add the child to the current line
        current_line.children_indices.push(index);
        current_line.total_height += total_child_height + (if current_line.children_indices.len() > 1 { container.get_styles().spacing.unwrap_or_default().spacing_y.value } else { 0.0 });  // Add space only after the first child
        current_line.max_width = current_line.max_width.max(child_size.width);
    }

    if !current_line.children_indices.is_empty() {
        lines.push(current_line);
    }

    lines
}

struct Line {
    children_indices: Vec<usize>,  // Store indices to the children
    total_height: f32,
    max_width: f32,
}


// Tests
#[cfg(test)]
mod tests {
    use crate::rendering::elements::{common_types::OptionalSize, styles::{Dimension, Margin, Padding, Spacing, Styles, Unit}};

    use super::*; 
    
    #[test]
    fn test_allocate_space_to_column_flex_wrap() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding {
                top: Dimension { value: 10.0, unit: Unit::Px },
                left: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut child = Container::new();
        child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 50.0, unit: Unit::Px }),
            height: Some(Dimension { value: 100.0, unit: Unit::Px })
        });
        child.set_styles(Styles {
            margin: Some(Margin {
                top: Dimension { value: 5.0, unit: Unit::Px },
                bottom: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        container.add_child(Box::new(child));

        let allocated_position = Position { x: 0.0, y: 0.0 };
        let allocated_size = Size { width: 100.0, height: 300.0 };
        
        // Act
        allocate_space_to_column_flex_wrap(&mut container, allocated_position, allocated_size);

        // Assert
        let allocated_child = &container.children[0];
        assert_eq!(allocated_child.get_position().y, 15.0); // 10 padding + 5 margin
        assert_eq!(allocated_child.get_position().x, 5.0);  // 5 padding
        assert_eq!(allocated_child.get_size().height, 100.0);
        assert_eq!(allocated_child.get_size().width, 50.0);
    }

    #[test]
    fn test_group_children_into_columns() {
        // Arrange
        let mut container = Container::new();
        container.set_styles(Styles {
            padding: Some(Padding {
                top: Dimension { value: 10.0, unit: Unit::Px },
                bottom: Dimension { value: 10.0, unit: Unit::Px },
                ..Default::default()
            }),
            spacing: Some(Spacing {
                spacing_y: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut first_child = Container::new();
        first_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 20.0, unit: Unit::Px }),
            height: Some(Dimension { value: 100.0, unit: Unit::Px })
        });
        first_child.set_styles(Styles {
            margin: Some(Margin {
                top: Dimension { value: 5.0, unit: Unit::Px },
                bottom: Dimension { value: 5.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut second_child = Container::new();
        second_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 25.0, unit: Unit::Px }),
            height: Some(Dimension { value: 150.0, unit: Unit::Px })
        });
        second_child.set_styles(Styles {
            margin: Some(Margin {
                top: Dimension { value: 3.0, unit: Unit::Px },
                bottom: Dimension { value: 3.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        let mut third_child = Container::new();
        third_child.set_requested_size(OptionalSize {
            width: Some(Dimension { value: 30.0, unit: Unit::Px }),
            height: Some(Dimension { value: 200.0, unit: Unit::Px })
        });
        third_child.set_styles(Styles {
            margin: Some(Margin {
                top: Dimension { value: 2.0, unit: Unit::Px },
                bottom: Dimension { value: 2.0, unit: Unit::Px },
                ..Default::default()
            }),
            ..Default::default()
        });

        container.add_child(Box::new(first_child));
        container.add_child(Box::new(second_child));
        container.add_child(Box::new(third_child));

        let allocated_size = Size { width: 100.0, height: 300.0 }; // Constraint the height to 300.0 to force wrapping.

        // Act
        let columns = group_children_into_column_lines(&container, allocated_size);

        // Assert
        assert_eq!(columns.len(), 2);  // Expect two columns due to height constraints.
        assert_eq!(columns[0].children_indices, vec![0, 1]);  // First and second child fit on the first column.
        assert_eq!(columns[1].children_indices, vec![2]);  // Third child wraps to the next column.
        assert!(columns[0].total_height > 0.0 && columns[1].total_height > 0.0);
        assert!(columns[0].max_width == 25.0 && columns[1].max_width == 30.0);  // Check max width of each column.
    }
}
