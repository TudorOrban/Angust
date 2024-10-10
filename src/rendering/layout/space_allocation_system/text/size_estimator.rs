use crate::rendering::{elements::{common_types::Size, element::Element, text::Text}, rendering_interface::custom_to_skia_types_mapper::estimate_text_size};


pub fn estimate_text_element_size(text_element: &Text) -> Size {
    estimate_text_size(
        &text_element.get_content(),
        text_element.get_styles().font_size.unwrap_or_default().value,
        text_element.get_styles().font_weight.unwrap_or_default(),
        text_element.get_styles().font_family.unwrap_or_default(),
        text_element.get_styles().font_style.unwrap_or_default(),
    )
} 