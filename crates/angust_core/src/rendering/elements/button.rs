use std::{any::Any, collections::HashMap};

use skia_safe::{Canvas, Color, Point};

use crate::{parsing::directive::for_parser::ForLoopContext, rendering::{layout::size_estimation_system::effective_size_estimator, rendering_interface::element_renderer::ElementRenderer}};

use super::{
    common_types::{OptionalSize, Position, Size}, 
    component::component::ComponentInterface, 
    container::Container, element::{Element, ElementType, EventType}, 
    element_id_generator::ElementIDGenerator, 
    styles::Styles
};

pub struct Button {
    _id: String,

    container: Option<Vec<Box<dyn Element>>>, // Only one container is allowed
    pub on_click_handler_name: Option<String>,
    pub loop_contexts: Vec<ForLoopContext>,

    position: Position,
    size: Size,
    styles: Styles,
    natural_size: Size,
    requested_size: OptionalSize,
}

impl Button {
    pub fn new(
        on_click_handler_name: Option<String>,
        loop_contexts: Option<Vec<ForLoopContext>>,
        container: Option<Container>, 
        styles: Option<Styles>
    ) -> Self {
        let id = ElementIDGenerator::get();

        let container_vec = if let Some(container_child) = container {
            Some(vec![Box::new(container_child) as Box<dyn Element>])
        } else {
            None
        };

        Self {
            _id: id,
            container: container_vec,
            on_click_handler_name,
            loop_contexts: loop_contexts.unwrap_or(vec![]),
            position: Position::default(),
            size: Size::default(),
            styles: styles.unwrap_or_default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
        }
    }
    
    fn get_children(&self) -> Option<&Vec<Box<dyn Element>>> {
        self.container.as_ref()
    }

    // Utils    
    fn is_position_within_bounds(&self, point: skia_safe::Point) -> bool {
        point.x >= self.position.x && point.x <= self.position.x + self.size.width &&
        point.y >= self.position.y && point.y <= self.position.y + self.size.height
    }
}

impl Element for Button {
    fn render(&self, canvas: &Canvas) {
        let has_children = if let Some(child_container) = self.get_children() {
            if child_container.len() != 1 {
                false
            } else {
                true
            }
        } else { false };
        if !has_children {
            return;
        }
        
        ElementRenderer::render_element(
            canvas,
            self.position,
            self.size,
            self.styles.background_color.unwrap_or(Color::TRANSPARENT),
            self.styles.border.unwrap_or_default().width,
            self.styles.border.unwrap_or_default().color,
        );

        if let Some(child_container) = self.get_children() {
            if let Some(child_element) = child_container.get(0) {
                child_element.render(canvas);
            }
        }
    }

    fn update(&mut self) {}

    fn handle_event(&mut self, _: Point, _: &EventType) {
        
    }

    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<EventPropagationData> {
        let mut event_targets = Vec::new();

        // Check if the cursor_position is within the bounds of the button
        let is_position_within_bounds = self.is_position_within_bounds(cursor_position);
        if !is_position_within_bounds {
            return event_targets;
        }

        if self.on_click_handler_name.is_none() {
            return event_targets;
        }
        let handler_name = self.on_click_handler_name.as_ref().unwrap();
        
        match event_type {
            EventType::MouseClick => {
                event_targets.push(EventPropagationData { handler_name: handler_name.clone(), for_loop_contexts: self.loop_contexts.clone() });
            },
            _ => {}
        }

        // TODO: Propagate to children as well

        event_targets
    }


    fn set_id(&mut self, id: String) {
        self._id = id;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn set_styles(&mut self, styles: Styles) {
        self.styles = styles;
    }

    fn add_child(&mut self, container: Box<dyn Element>) {
        // Only one container is allowed
        if self.container.is_some() {
            return;
        }

        self.container = Some(vec![container]);
    }

    fn get_id(&self) -> String {
        self._id.clone()
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Button
    }

    fn get_name(&self) -> String {
        "button".to_string()
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> { 
        self.container.as_mut()
    }
    
    // Custom component
    fn get_component_interface(&mut self) -> Option<&mut dyn ComponentInterface> {
        None
    }

    fn initialize(&mut self, _: HashMap<String, Box<dyn Any>>) {
        // Nothing for now (implemented for components only)
    }

    fn handle_route_change(&mut self, route: &String, component_name: &String) {
        if let Some(child_container) = self.get_children_mut() {
            if child_container.len() != 1 {
                return;
            }
            if let Some(child_element) = child_container.get_mut(0) {
                child_element.handle_route_change(route, component_name);
            }
        }
    }

    // Layout system
    fn set_natural_size(&mut self, size: Size) {
        self.natural_size = size;
    }

    fn set_requested_size(&mut self, requested_size: OptionalSize) {
        self.requested_size = requested_size;
    }

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize { 
        self.requested_size 
    }

    fn get_effective_size(&self) -> Size {
        effective_size_estimator::estimate_effective_size(&self.get_requested_size(), &self.get_natural_size())
    }

    fn is_text_wrapper(&self) -> bool { false }

    
    fn estimate_sizes(&mut self) {
        let (mut natural_size, mut requested_size) = (None, None);

        if let Some(child_container) = self.get_children_mut() {
            if child_container.len() != 1 {
                return;
            }
            if let Some(child_element) = child_container.get_mut(0) {
                child_element.estimate_sizes(); 

                natural_size = Some(child_element.get_natural_size());
                requested_size = Some(child_element.get_requested_size());
            }
        } 

        if let Some(size) = natural_size {
            self.set_natural_size(size);
        }
        if let Some(size) = requested_size {
            self.set_requested_size(size);
        }
    }

    // Second pass: Traverse the DOM from root to leaves and allocate space to each container.
    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = allocated_position;
        self.size = allocated_size;

        if let Some(child_container) = self.get_children_mut() {
            if child_container.len() != 1 {
                return;
            }
            if let Some(child_element) = child_container.get_mut(0) {
                child_element.allocate_space(allocated_position, allocated_size);
            }
        } 
    }
    
    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }

    // Reactivity
    fn react_to_state_change(&mut self, component_id: String) {
        if let Some(child_container) = self.get_children_mut() {
            if child_container.len() != 1 {
                return;
            }
            if let Some(child_element) = child_container.get_mut(0) {
                child_element.react_to_state_change(component_id);
            }
        }
    }
}


pub struct EventPropagationData {
    pub handler_name: String,
    pub for_loop_contexts: Vec<ForLoopContext>,
}