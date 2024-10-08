use skia_safe::{Canvas, Color, Paint, PaintStyle, Point, Rect};

use crate::rendering::elements::{common_types::{Position, Size}, styles::{Dimension, Directions}};



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
        let thumb_size = Size {
            width: size.width * 0.2,
            height: size.height * 0.8,
        };
        let thumb_position = Position {
            x: position.x + current_position * size.width - thumb_size.width / 2.0,
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
}