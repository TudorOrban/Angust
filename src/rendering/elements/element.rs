use skia_safe::{Canvas, Point};

use super::{common_types::{OptionalSize, Position, Size}, styles::Styles};

// This is the trait that all Angust elements must implement.
pub trait Element {
    fn render(&self, canvas: &Canvas);
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

    fn set_id(&mut self, id: String);
    fn set_position(&mut self, position: Position);
    fn set_size(&mut self, size: Size);
    fn add_child(&mut self, child: Box<dyn Element>);

    fn get_id(&self) -> String;
    fn get_element_type(&self) -> ElementType;
    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;
    fn get_styles(&self) -> Styles;

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>>;
    
    // Layout system
    fn set_natural_size(&mut self, size: Size);
    fn set_requested_size(&mut self, optional_size: OptionalSize);
    fn get_natural_size(&self) -> Size;
    fn get_requested_size(&self) -> OptionalSize;
    fn get_effective_size(&self) -> Size;
    fn is_text_wrapper(&self) -> bool;
    fn estimate_sizes(&mut self);
    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size);
    fn layout(&mut self, allocated_position: Position, allocated_size: Size);
}


#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
    MouseClick,
    MouseMove,
    MouseDown,
    MouseDrag,
    MouseUp,
    MouseRoll(f32),
    KeyPress(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementType {
    Container,
    Button,
    Text,
    Image,

}