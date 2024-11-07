use image::DynamicImage;
use skia_safe::{Canvas, Color, Paint, PaintStyle, Point, Rect, TextBlob};

use crate::rendering::elements::{common_types::{Position, Size}, styles::{Dimension, Directions, FontFamily, FontStyle as CustomFontStyle, FontWeight}};

use super::skia_boundary::{self, get_skia_font_by_styles};


pub struct ElementRenderer {
    
}

impl ElementRenderer {
    pub fn render_element(
        canvas: &Canvas,
        position: Position, 
        size: Size, 
        background_color: Color,
        border_width: Dimension,
        border_color: Color
    ) {
        if (size.width <= 0.0) || (size.height <= 0.0) {
            return;
        }

        let row_rect = Rect::from_point_and_size(
            Point::new(position.x,
                       position.y),
            (size.width,
             size.height)
        );
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_color(background_color);
        canvas.draw_rect(row_rect, &paint);
         
        if border_width.value == 0.0 {
            return; // Draw all borders for now for debugging
        }
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(border_width.value);
        paint.set_color(border_color);
        canvas.draw_rect(row_rect, &paint);
    }

    pub fn render_scrollbar(
        canvas: &Canvas,
        position: Position,
        size: Size,
        _: Directions,
        current_position: f32, // Between 0.0 and 1.0
        thumb_scrollbar_width_ratio: f32,
    ) {
        // Draw outer rectangle
        let mut paint = Paint::default();
        let scrollbar_rect = Rect::from_point_and_size(
            Point::new(position.x,
                       position.y),
            (size.width,
             size.height)
        );
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::from_argb(255, 200, 200, 200));

        canvas.draw_rect(scrollbar_rect, &paint);

        // Draw thumb
        let thumb_width = size.width * thumb_scrollbar_width_ratio;
        let thumb_size = Size {
            width: thumb_width,
            height: size.height * 0.8,
        };
        let max_left_position = position.x;
        let max_right_position = position.x + size.width - thumb_width;

        let thumb_position = Position {
            x: max_left_position + (current_position * (max_right_position - max_left_position)),
            y: position.y + size.height * 0.1,
        };

        let thumb_rect = Rect::from_point_and_size(
            Point::new(thumb_position.x,
                       thumb_position.y),
            (thumb_size.width,
             thumb_size.height)
        );

        paint.set_color(Color::from_argb(255, 100, 100, 100));
        canvas.draw_rect(thumb_rect, &paint);
    }

    pub fn render_multi_line_text(
        canvas: &Canvas,
        position: Position, 
        lines: Vec<String>,
        text_color: Color,
        font_size: f32,
        font_weight: FontWeight,
        font_family: FontFamily,
        font_style: CustomFontStyle,
    ) {
        let font = get_skia_font_by_styles(font_size, font_weight, font_family, font_style);
        let mut y_offset = position.y;

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(text_color);

        for line in lines {
            if let Some(blob) = TextBlob::from_text(line.clone(), &font) {
                canvas.draw_text_blob(&blob, Point::new(position.x, y_offset), &paint);
                let (_, rect) = font.measure_str(line, Some(&paint));
                y_offset += rect.height();
            }
        }
    }

    pub fn render_image(
        image: &DynamicImage,
        canvas: &Canvas,
        position: Position,
        size: Size,
    ) {
        let skia_image = skia_boundary::dynamic_image_to_skia_image(&image);
        if let Some(skia_image) = skia_image {
            // Calculate the drawing destination based on position and size
            let src_rect = skia_safe::Rect::from_wh(skia_image.width() as f32, skia_image.height() as f32);
            let dst_rect = skia_safe::Rect::from_xywh(
                position.x, 
                position.y, 
                size.width, 
                size.height
            );

            // Draw the image
            canvas.draw_image_rect(
                &skia_image, 
                Some((&src_rect, skia_safe::canvas::SrcRectConstraint::Fast)), 
                dst_rect, 
                &skia_safe::Paint::default()
            );
        }
    }
}
