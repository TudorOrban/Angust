use crate::rendering::elements::{common_types::Size, styles::{FlexWrap, Overflow}};



pub fn determine_allocated_size(
    flex_wrap: FlexWrap,
    overflow: Overflow,
    child_effective_size: Size,
    allocated_size: Size,
) -> Size {
    if flex_wrap != FlexWrap::NoWrap {
        return child_effective_size; // To be implemented later
    }

    if overflow == Overflow::Visible {
        return child_effective_size; // No need to clip
    }

    let clipped_width = child_effective_size.width.min(allocated_size.width);
    
    Size {
        width: clipped_width,
        height: child_effective_size.height,
    }
}
