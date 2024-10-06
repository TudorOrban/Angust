use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, element::Element, styles::FlexDirection};


/*
 * Determine the *natural* and *requested* size of the container 
 * based on the children's *effective* sizes (i.e. requested if specified, natural otherwise).
 */
pub fn estimate_parent_container_sizes(container: &mut Container) {
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;

    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();
    
    for child in &container.children {
        let child_effective_size = child.get_effective_size();

        match flex_direction {
            FlexDirection::Row => {
                width += child_effective_size.width;
                height = height.max(child_effective_size.height);
            },
            FlexDirection::Column => {
                width = width.max(child_effective_size.width);
                height += child_effective_size.height;
            },
        }
    }
    
    container.set_natural_size(Size { width, height });
    
    let sizing_policy = container.get_styles().sizing_policy.unwrap_or_default();
    container.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height });
}

pub fn estimate_leaf_container_sizes(container: &mut Container) {
    if let Some(sizing_policy) = container.get_styles().sizing_policy {
        container.set_requested_size(OptionalSize {
            width: sizing_policy.width,
            height: sizing_policy.height,
        });
        container.set_natural_size(Size::default()); // Size 0 for leaf containers
    }
}