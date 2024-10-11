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
    
    let thumb_width = container.get_size().width * container.scrollbar_state.thumb_scrollbar_width_ratio;
    let track_width = container.get_size().width;

    let container_rect = Rect::from_point_and_size(
        Point::new(container.get_position().x, container.get_position().y),
        (container.get_size().width, container.get_size().height)
    );
    let scrollbar_rect = Rect::from_point_and_size(
        Point::new(container.get_position().x, container.get_position().y + container.get_size().height - container.scrollbar_state.scrollbar_thickness),
        (container.get_size().width, container.scrollbar_state.scrollbar_thickness)
    );
    
    let thumb_position_x = container.get_position().x + container.scrollbar_state.current_scroll_position.x * (track_width - thumb_width);
    let thumb_rect = Rect::from_point_and_size(
        Point::new(thumb_position_x, container.get_position().y + container.get_size().height - container.scrollbar_state.scrollbar_thickness),
        (thumb_width, container.scrollbar_state.scrollbar_thickness)
    );

    match event_type {
        EventType::MouseDown => {
            if thumb_rect.contains(cursor_position) {
                container.scrollbar_state.is_dragging = true;
                container.scrollbar_state.drag_start_position = Position {
                    x: cursor_position.x - thumb_position_x,
                    y: cursor_position.y
                };
            } else if scrollbar_rect.contains(cursor_position) {
                // Calculate new thumb position based on where the user clicked
                let new_thumb_center = cursor_position.x - container.get_position().x;
                let new_position = (new_thumb_center - thumb_width / 2.0) / (track_width - thumb_width);
                container.scrollbar_state.current_scroll_position.x = new_position.clamp(0.0, 1.0);

                container.allocate_space(container.get_position(), container.get_size());
            }
        }
        EventType::MouseDrag => {
            if container.scrollbar_state.is_dragging {
                let adjusted_cursor_x = cursor_position.x - container.scrollbar_state.drag_start_position.x;
                let new_x = adjusted_cursor_x / (track_width - thumb_width);
                container.scrollbar_state.current_scroll_position.x = new_x.clamp(0.0, 1.0);
    
                container.allocate_space(container.get_position(), container.get_size());
            }
        }
        EventType::MouseUp => {
            container.scrollbar_state.is_dragging = false;
        }
        EventType::MouseRoll(delta) => {
            if container_rect.contains(cursor_position) {
                let adjustment_factor = 0.01;
                let new_position = container.scrollbar_state.current_scroll_position.x - delta * adjustment_factor;
                container.scrollbar_state.current_scroll_position.x = new_position.clamp(0.0, 1.0);
                
                container.allocate_space(container.get_position(), container.get_size());
            }
        }
        _ => {}
    }
}