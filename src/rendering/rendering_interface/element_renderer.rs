use skia_safe::{Canvas, Color, Paint, PaintStyle, Point, Rect};

use crate::rendering::elements::common_types::{Position, Size};



pub struct ElementRenderer {
    
}

impl ElementRenderer {
    pub fn render_element(
        canvas: &Canvas,
        position: Position, 
        size: Size, 
        background_color: Color,
        border_width: f32,
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
    
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(border_width);
        paint.set_color(border_color);
        canvas.draw_rect(row_rect, &paint);
    }
}