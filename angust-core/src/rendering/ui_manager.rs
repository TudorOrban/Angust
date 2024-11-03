use super::elements::{common_types::{Position, Size}, element::{Element, EventType}};

pub struct UIManager {
    root_element: Box<dyn Element>,
}

impl UIManager {
    pub fn new(root_element: Box<dyn Element>) -> Self {
        Self { root_element }
    }

    pub fn render(&mut self, canvas: &skia_safe::Canvas) {
        self.root_element.render(canvas);
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        self.root_element.update();
    }

    pub fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        self.root_element.handle_event(cursor_position, event_type);
    }

    pub fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<String> {
        self.root_element.propagate_event(cursor_position, event_type)
    }

    pub fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.root_element.layout(allocated_position, allocated_size);
    }

    pub fn react_to_state_change(&mut self, component_id: String) {
        self.root_element.react_to_state_change(component_id);
    }

    pub fn handle_route_change(&mut self, route: &String, component_name: &String) {
        self.root_element.handle_route_change(route, component_name);
    }
}
