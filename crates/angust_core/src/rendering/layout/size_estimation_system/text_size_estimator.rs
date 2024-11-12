use crate::rendering::{
    elements::{
        common_types::Size, 
        element::Element, 
        styles::{Dimension, Unit}, 
        text::Text
    }, 
    rendering_interface::skia_boundary
};

/*
 * Function to estimate the size of a Text element, using styles and skia
 */
pub fn estimate_text_element_size(text_element: &Text) -> Size {
    skia_boundary::estimate_text_size(
        &text_element.get_content(),
        text_element.get_styles().font_size.unwrap_or(Dimension { value: 16.0, unit: Unit::Px }).value,
        text_element.get_styles().font_weight.unwrap_or_default(),
        text_element.get_styles().font_family.unwrap_or_default(),
        text_element.get_styles().font_style.unwrap_or_default(),
    )
} 

pub fn determine_text_element_lines(text_element: &Text) -> Vec<String> {
    skia_boundary::determine_text_lines(
        &text_element.get_content(),
        text_element.get_styles().font_size.unwrap_or(Dimension { value: 16.0, unit: Unit::Px }).value,
        text_element.get_styles().font_weight.unwrap_or_default(),
        text_element.get_styles().font_family.unwrap_or_default(),
        text_element.get_styles().font_style.unwrap_or_default(),
        text_element.get_size().width,
    )
}