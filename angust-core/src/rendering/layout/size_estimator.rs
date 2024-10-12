use crate::rendering::elements::{common_types::{OptionalSize, Size}, container::Container, element::Element, styles::FlexDirection};


/*
 * Determine the *natural* and *requested* size of the container 
 * based on the children's *effective* sizes (i.e. requested if specified, natural otherwise).
 */
pub fn estimate_parent_container_sizes(container: &mut Container) {
    let flex_direction = container.get_styles().flex_direction.unwrap_or_default();

    let parent_padding = container.get_styles().padding.unwrap_or_default();
    let spacing = container.get_styles().spacing.unwrap_or_default();
    let mut width: f32 = parent_padding.horizontal();
    let mut height: f32 = parent_padding.vertical();

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
                height = height.max(child_effective_size.height + parent_padding.vertical() + margin.vertical());
            },
            FlexDirection::Column => {
                if index > 0 {
                    height += spacing.spacing_y.value;
                }
                height += margin.top.value + child_effective_size.height + margin.bottom.value;
                width = width.max(child_effective_size.width + parent_padding.horizontal() + margin.horizontal());
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