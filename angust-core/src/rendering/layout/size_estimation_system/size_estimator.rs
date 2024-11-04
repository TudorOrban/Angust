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