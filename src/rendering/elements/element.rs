use skia_safe::{Canvas, Point};

use super::{common_types::{Position, Size}, styles::Styles};

// This is the trait that all Reast elements must implement.
pub trait Element {
    fn render(&self, canvas: &Canvas);
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

    fn get_id(&self) -> String;
    fn get_element_type(&self) -> ElementType;
    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;
    fn get_styles(&self) -> Styles;

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>>;
    
    // Layout system
    fn compute_allocation_plan(&mut self);
    fn enact_allocation_plan(&mut self, allocated_position: Position, allocated_size: Size);
}


pub enum EventType {
    MouseClick,
    MouseMove,
    KeyPress(char),
}

pub enum ElementType {
    Container,
    Button,
}