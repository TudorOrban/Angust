use skia_safe::{font_style::{Slant, Weight, Width}, Font, FontMgr, FontStyle, Paint};

use crate::rendering::elements::{common_types::Size, styles::{FontFamily, FontStyle as CustomFontStyle, FontWeight as CustomFontWeight}};


pub fn estimate_text_size(
    text: &str, 
    font_size: f32, 
    font_weight: CustomFontWeight, 
    font_family: FontFamily, 
    font_style: CustomFontStyle
) -> Size {
    let font = get_skia_font_by_styles(font_size, font_weight, font_family, font_style);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let (_, rect) = font.measure_str(text, Some(&paint));

    Size {
        width: rect.width(),
        height: rect.height(),
    }
}

pub fn determine_text_lines(
    text_content: &str,
    font_size: f32, 
    font_weight: CustomFontWeight, 
    font_family: FontFamily, 
    font_style: CustomFontStyle,
    max_width: f32,
) -> Vec<String> {
    let font = get_skia_font_by_styles(font_size, font_weight, font_family, font_style);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    let lines = calculate_text_lines(text_content, &font, &paint, max_width);

    lines
}

fn calculate_text_lines(
    text_content: &str,
    font: &Font,
    paint: &Paint,
    max_width: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text_content.split_whitespace() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let (_, rect) = font.measure_str(&test_line, Some(paint));

        if rect.width() > max_width {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}


pub fn get_skia_font_by_styles(
    font_size: f32, 
    font_weight: CustomFontWeight, 
    font_family: FontFamily, 
    font_style: CustomFontStyle
) -> Font {
    let font_mgr = FontMgr::default();
    let slant: Slant = map_custom_to_skia_font_style(&font_style);
    let weight = map_custom_to_skia_font_weight(&font_weight);
    let font_style = FontStyle::new(weight, Width::from(20), slant);
    
    let typeface = font_mgr.match_family_style(font_family.to_string(), font_style)
        .expect("Unable to create typeface");

    Font::new(typeface, font_size)
}

pub fn map_custom_to_skia_font_style(font_style: &CustomFontStyle) -> Slant {
    match font_style {
        CustomFontStyle::Normal => Slant::Upright,
        CustomFontStyle::Italic => Slant::Italic,
        CustomFontStyle::Oblique => Slant::Oblique,
    }
}

pub fn map_custom_to_skia_font_weight(font_weight: &CustomFontWeight) -> Weight {
    Weight::from(font_weight.to_number())
}

