use crate::rendering::elements::{common_types::Size, container::Container, element::Element, styles::FlexDirection};



/*
 * Estimate the size of the container based on the children's sizes.
 * Should be called only after the children sizes have been estimated.
 */
pub fn estimate_parent_container_size(container: &mut Container) {
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;

    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    for child in &container.children {
        let child_size = child.get_size();

        match flex_direction {
            FlexDirection::Row => {
                width += child_size.width;
                height = height.max(child_size.height);
            },
            FlexDirection::Column => {
                width = width.max(child_size.width);
                height += child_size.height;
            },
        }
    }

    container.set_size(Size { width, height });
}

pub fn estimate_leaf_container_size(container: &mut Container) {
    if let Some(sizing_policy) = container.get_styles().sizing_policy {
        let requested_size = Size {
            width: sizing_policy.width.unwrap_or_default().value,
            height: sizing_policy.height.unwrap_or_default().value,
        };
        
        container.set_requested_size(requested_size);
        container.set_natural_size(requested_size); // TODO: Implement the logic for the natural size
    }
}