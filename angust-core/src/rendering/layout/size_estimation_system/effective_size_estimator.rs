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
