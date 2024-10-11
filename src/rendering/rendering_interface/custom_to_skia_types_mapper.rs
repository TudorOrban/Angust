use skia_safe::{font_style::{Slant, Weight, Width}, Font, FontMgr, FontStyle, Paint};

use crate::rendering::elements::{common_types::Size, styles::{FontFamily, FontStyle as CustomFontStyle, FontWeight as CustomFontWeight, WhiteSpace}};


pub fn estimate_text_size(
    text: &str, 
    white_space: WhiteSpace,
    font_size: f32, 
    font_weight: CustomFontWeight, 
    font_family: FontFamily, 
    font_style: CustomFontStyle
) -> Size {
    let font_mgr = FontMgr::default();
    let slant: Slant = map_custom_to_skia_font_style(&font_style);
    let weight = map_custom_to_skia_font_weight(&font_weight);
    let font_style = FontStyle::new(weight, Width::from(20), slant);
    
    let typeface = font_mgr.match_family_style(font_family.to_string(), font_style)
        .expect("Unable to create typeface");

    let font = Font::new(typeface, font_size);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let (_, rect) = font.measure_str(text, Some(&paint));

    Size {
        width: rect.width(),
        height: rect.height(),
    }
}

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

