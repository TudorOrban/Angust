use skia_safe::{font_style::{Slant, Width}, Canvas, Color, Font, FontMgr, FontStyle, Paint, PaintStyle, Point, Rect, TextBlob};

use crate::rendering::elements::{common_types::{Position, Size}, styles::{Dimension, Directions, FontFamily, FontStyle as CustomFontStyle, FontWeight, WhiteSpace}};

use super::skia_boundary::{get_skia_font_by_styles, map_custom_to_skia_font_style, map_custom_to_skia_font_weight};


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
            // return; // Draw all borders for now
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
        directions: Directions,
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
            y: position.y,
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
        size: Size, 
        lines: Vec<String>,
        text_color: Color,
        white_space: WhiteSpace,
        font_size: f32,
        font_weight: FontWeight,
        font_family: FontFamily,
        font_style: CustomFontStyle,
        text_content: String,
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
}