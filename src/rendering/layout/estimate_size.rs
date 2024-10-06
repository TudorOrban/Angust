use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, element::Element, styles::{Dimension, FlexDirection}};



/*
 * Estimate the size of the container based on the children's sizes.
 * Should be called only after the children sizes have been estimated.
 */
pub fn estimate_parent_container_size(container: &mut Container) {
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;

    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();
    
    for child in &container.children {
        let child_natural_size = child.get_natural_size();

        match flex_direction {
            FlexDirection::Row => {
                width += child_natural_size.width;
                height = height.max(child_natural_size.height);
            },
            FlexDirection::Column => {
                width = width.max(child_natural_size.width);
                height += child_natural_size.height;
            },
        }
    }
    
    container.set_natural_size(Size { width, height });
    
    let sizing_policy = container.get_styles().sizing_policy.unwrap_or_default();

    container.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height });
}

pub fn estimate_leaf_container_size(container: &mut Container) {
    if let Some(sizing_policy) = container.get_styles().sizing_policy {
        let requested_size = Size {
            width: sizing_policy.width.unwrap_or_default().value,
            height: sizing_policy.height.unwrap_or_default().value,
        };
        
        container.set_requested_size(OptionalSize {
            width: Some(Dimension {
                value: requested_size.width,
                ..Default::default()
            }),
            height: Some(Dimension {
                value: requested_size.height,
                ..Default::default()
            }),
        });
        container.set_natural_size(requested_size); // TODO: Implement the logic for the natural size
    }
}