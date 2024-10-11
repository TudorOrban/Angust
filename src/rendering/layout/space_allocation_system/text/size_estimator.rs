use crate::rendering::{elements::{common_types::Size, element::Element, styles::{Dimension, Unit}, text::Text}, rendering_interface::custom_to_skia_types_mapper::estimate_text_size};


pub fn estimate_text_element_size(text_element: &Text) -> Size {
    estimate_text_size(
        &text_element.get_content(),
        text_element.get_styles().white_space.unwrap_or_default(),
        text_element.get_styles().font_size.unwrap_or(Dimension { value: 16.0, unit: Unit::Px}).value,
        text_element.get_styles().font_weight.unwrap_or_default(),
        text_element.get_styles().font_family.unwrap_or_default(),
        text_element.get_styles().font_style.unwrap_or_default(),
    )
} 