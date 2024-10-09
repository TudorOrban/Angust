use skia_safe::{Contains, Point, Rect};

use crate::rendering::elements::{common_types::Position, container::Container, element::{Element, EventType}};


pub fn handle_scrollbar_movement(
    container: &mut Container,
    cursor_position: Point,
    event_type: &EventType,
) {
    if !container.scrollbar_state.is_overflowing.horizontal {
        return; // Implement vertical scrollbar handling later
    }

    let thumb_scrollbar_width_ratio = 0.2;
    
    let scrollbar_rect = Rect::from_point_and_size(
        Point::new(container.get_position().x, container.get_position().y + container.get_size().height - 10.0),
        (container.get_size().width, 10.0)
    );
    
    if scrollbar_rect.contains(cursor_position) || container.scrollbar_state.is_dragging {
        match event_type {
            EventType::MouseDown => {
                container.scrollbar_state.is_dragging = true;
                container.scrollbar_state.drag_start_position = Position { 
                    x: cursor_position.x - (container.scrollbar_state.current_scroll_position.x * container.get_size().width),
                    y: cursor_position.y
                };
            },
            EventType::MouseDrag if container.scrollbar_state.is_dragging => {
                let new_x = (cursor_position.x - container.scrollbar_state.drag_start_position.x) / (container.get_size().width * (1.0 - thumb_scrollbar_width_ratio));
                container.scrollbar_state.current_scroll_position.x = new_x.clamp(0.0, 1.0);
            },
            EventType::MouseUp => {
                container.scrollbar_state.is_dragging = false;
            },
            _ => {}
        }
    }
}