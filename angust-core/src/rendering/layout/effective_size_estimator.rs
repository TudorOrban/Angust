use crate::rendering::elements::{common_types::{OptionalSize, Size}, styles::{Dimension, Unit}};


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