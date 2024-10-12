use image::{DynamicImage, GenericImageView};
use skia_safe::{font_style::{Slant, Weight, Width}, Bitmap, ColorType, Font, FontMgr, FontStyle, Paint};

use crate::rendering::elements::{common_types::Size, styles::{FontFamily, FontStyle as CustomFontStyle, FontWeight as CustomFontWeight}};

// Text
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

// Images
pub fn dynamic_image_to_skia_image(image: &DynamicImage) -> Option<skia_safe::Image> {
    let (width, height) = image.dimensions();
    let image_info = skia_safe::ImageInfo::new(
        skia_safe::ISize::new(width as i32, height as i32),
        ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );

    let pixel_data = image.to_rgba8().into_raw();

    // Create a Skia bitmap from the pixel data
    let mut bitmap = Bitmap::new();
    let row_bytes = (4 * width) as usize;
    unsafe {
        let pixel_ptr = pixel_data.as_ptr() as *const _ as *mut _;
        bitmap.install_pixels(&image_info, pixel_ptr, row_bytes);
    }
    
    // Convert the bitmap to a Skia image
    skia_safe::images::raster_from_data(&image_info, skia_safe::Data::new_copy(&pixel_data), row_bytes)
}