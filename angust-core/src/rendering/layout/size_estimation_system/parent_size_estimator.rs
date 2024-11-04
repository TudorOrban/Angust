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