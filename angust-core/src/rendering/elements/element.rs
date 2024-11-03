use std::{any::Any, collections::HashMap};

use skia_safe::{Canvas, Point};

use super::{
    common_types::{OptionalSize, Position, Size}, 
    component::component::ComponentInterface, 
    styles::Styles
};

// This is the trait that all Angust elements must implement.
pub trait Element {
    // Core
    fn render(&self, canvas: &Canvas);
    fn update(&mut self);
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType);

    // Experimental: gather all names of event handlers that should be called by a parent Custom Component
    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<String>;

    // Getters and setters
    fn set_id(&mut self, id: String);
    fn set_position(&mut self, position: Position);
    fn set_size(&mut self, size: Size);
    fn set_styles(&mut self, styles: Styles);
    fn add_child(&mut self, child: Box<dyn Element>);

    fn get_id(&self) -> String;
    fn get_element_type(&self) -> ElementType;
    fn get_name(&self) -> String;
    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;
    fn get_styles(&self) -> Styles;

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>>;

    // Custom components
    fn get_component_interface(&mut self) -> Option<&mut dyn ComponentInterface>;
    fn initialize(&mut self, inputs: HashMap<String, Box<dyn Any>>);
    fn handle_route_change(&mut self, route: &String, component_name: &String);

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

    // Reactivity
    fn react_to_state_change(&mut self, component_id: String);
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
    CustomComponent
}