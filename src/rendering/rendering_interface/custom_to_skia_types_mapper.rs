use skia_safe::font_style::{Slant, Weight};

use crate::rendering::elements::styles::{FontStyle as CustomFontStyle, FontWeight as CustomFontWeight};



pub fn map_custom_to_skia_font_style(font_style: &CustomFontStyle) -> Slant {
    match font_style {
        CustomFontStyle::Normal => Slant::Upright,
        CustomFontStyle::Italic => Slant::Italic,
        CustomFontStyle::Oblique => Slant::Oblique,
        _ => Slant::Upright,
    }
}

pub fn map_custom_to_skia_font_weight(font_weight: &CustomFontWeight) -> Weight {
    Weight::from(font_weight.to_number())
}