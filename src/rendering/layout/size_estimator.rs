use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, element::Element, styles::FlexDirection};


/*
 * Determine the *natural* and *requested* size of the container 
 * based on the children's *effective* sizes (i.e. requested if specified, natural otherwise).
 */
pub fn estimate_parent_container_sizes(container: &mut Container) {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    let padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let mut width: f32 = padding.left.value + padding.right.value;
    let mut height: f32 = padding.top.value;

    // Compute natural size of the container based on the children's effective sizes
    for (index, child) in container.children.iter_mut().enumerate() {
        let margin = child.get_styles().margin.unwrap_or_default();
        let child_effective_size = child.get_effective_size();

        match flex_direction {
            FlexDirection::Row => {
                if index > 0 {
                    width += spacing.spacing_x.value;
                }
                width += margin.left.value + child_effective_size.width + margin.right.value;
                height = height.max(child_effective_size.height + padding.top.value + padding.bottom.value + spacing.spacing_y.value * index as f32);
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
    container.set_natural_size(Size {
        width: container.get_styles().padding.unwrap_or_default().horizontal(),
        height: container.get_styles().padding.unwrap_or_default().vertical(),
    });
    
    if let Some(sizing_policy) = container.get_styles().sizing_policy {
        container.set_requested_size(OptionalSize {
            width: sizing_policy.width,
            height: sizing_policy.height,
        });
    }
}