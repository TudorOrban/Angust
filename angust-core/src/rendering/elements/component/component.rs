use std::collections::HashMap;

use crate::rendering::elements::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType}, element_id_generator::IDGenerator, styles::Styles};

pub struct Component<State> {
    _id: String,
    pub name: String,
    pub template_relative_path: String,
    pub content: Option<Box<dyn Element>>,
    pub position: Position,
    pub size: Size,
    pub natural_size: Size,
    pub requested_size: OptionalSize,
    pub styles: Styles,

    // User-defined state
    pub state: State,
    event_handlers: HashMap<String, Box<dyn FnMut(&mut State)>>,
}

impl<State> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        let id = IDGenerator::get();
        let mut component = Self {
            _id: id,
            name,
            template_relative_path: template_relative_path.clone(),
            content: None,
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            state,
            event_handlers: HashMap::new(),
        };
        component.initialize(template_relative_path);
        component
    }

    fn initialize(&mut self, template_relative_path: String) {
        // Load template
        println!("Loading template from {}", template_relative_path);
    }

    pub fn add_event_handler<F>(&mut self, event_name: String, handler: F)
    where
        F: 'static + FnMut(&mut State),
    {
        self.event_handlers.insert(event_name, Box::new(handler));
    }

    pub fn handle_event(&mut self, event: &str) {
        if let Some(handler) = self.event_handlers.get_mut(event) {
            handler(&mut self.state);
        }
    }
}

impl<State> Element for Component<State> {
    
    fn render(&self, canvas: &skia_safe::Canvas) {
        
    }

    fn update(&mut self) {
        
    }
    
    fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &crate::rendering::elements::element::EventType) {
        
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        
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

    fn set_natural_size(&mut self, size: Size) {
        self.natural_size = size;
    }

    fn set_requested_size(&mut self, optional_size: OptionalSize) {
        self.requested_size = optional_size;
    }

    fn is_text_wrapper(&self) -> bool {
        false
    }
    
    fn get_id(&self) -> String {
        self._id.clone()
    }
    
    fn get_element_type(&self) -> ElementType {
        ElementType::CustomComponent
    }
    
    fn get_position(&self) -> Position {
        self.position
    }
    
    fn get_size(&self) -> Size {
        self.size   
    }

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize {
        self.requested_size
    }

    fn get_effective_size(&self) -> Size {
        return self.size;
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }
    
    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        None // Placeholder
    }

    fn estimate_sizes(&mut self) {
        
    }

    fn allocate_space(&mut self, allocated_position: crate::rendering::elements::common_types::Position, allocated_size: crate::rendering::elements::common_types::Size) {
        
    }

    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }
}