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
    
    let scrollbar_rect = Rect::from_point_and_size(
        Point::new(container.get_position().x, container.get_position().y + container.get_size().height - 10.0),
        (container.get_size().width, 10.0)
    );

    if scrollbar_rect.contains(cursor_position) {
        match event_type {
            EventType::MouseDown => {
                container.scrollbar_state.is_dragging = true;
                container.scrollbar_state.drag_start_position = Position { x: cursor_position.x, y: cursor_position.y };
            },
            EventType::MouseDrag if container.scrollbar_state.is_dragging => {
                let drag_distance = cursor_position.x - container.scrollbar_state.drag_start_position.x;
                let max_scroll_distance = container.get_natural_size().width - scrollbar_rect.width();
                let scroll_distance = drag_distance / max_scroll_distance;
                container.scrollbar_state.current_scroll_position.x = scroll_distance;
            },
            EventType::MouseUp => {
                container.scrollbar_state.is_dragging = false;
            },
            _ => {}
        }
    }
}