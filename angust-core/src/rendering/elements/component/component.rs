use std::collections::HashMap;

use crate::rendering::{elements::{
    common_types::{OptionalSize, Position, Size}, 
    container::Container, 
    element::{Element, ElementType, EventType}, 
    element_id_generator::IDGenerator, 
    event_propagator, 
    styles::Styles
}, layout::effective_size_estimator};

use super::{component_state::ComponentState, template_loader};

pub struct Component<State: ComponentState> {
    _id: String,
    pub name: String,
    pub template_relative_path: String,
    pub content: Box<dyn Element>,
    position: Position,
    size: Size, 
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,

    // User-defined properties
    pub state: State,
    pub event_handlers: HashMap<String, Box<dyn FnMut(&mut State)>>,
}

impl<State: ComponentState> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        let mut component = Self {
            _id: IDGenerator::get(),
            name,
            template_relative_path,
            content: Box::new(Container::new()),
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            state,
            event_handlers: HashMap::new(),
        };

        component.initialize();

        component
    }

    fn initialize(&mut self) {
        self.load_component_template();
    }

    fn load_component_template(&mut self) {
        template_loader::load_component_template(self);
    }

    pub fn add_event_handler<F>(&mut self, event_name: String, handler: F)
    where
        F: 'static + FnMut(&mut State),
    {
        self.event_handlers.insert(event_name, Box::new(handler));
    }
}

impl<State: ComponentState> Element for Component<State> {
    
    fn render(&self, canvas: &skia_safe::Canvas) {
        self.content.render(canvas);
    }

    fn update(&mut self) {
        self.content.update();
    }
    
    fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        self.content.handle_event(cursor_position, event_type);
    }
    
    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<String> {
        let event_handler_names = event_propagator::propagate_event(self, cursor_position, event_type);
        
        for handler_name in event_handler_names.iter() {
            if let Some(handler) = self.event_handlers.get_mut(handler_name) {
                handler(&mut self.state);
            }
        }

        vec![]
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        self.content.add_child(child);
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

    fn set_styles(&mut self, styles: Styles) {
        self.styles = styles;
        self.content.set_styles(styles);
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
        effective_size_estimator::estimate_effective_size(&self.get_requested_size(), &self.get_natural_size())
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }
    
    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        return self.content.get_children_mut();
    }

    fn estimate_sizes(&mut self) {
        self.content.estimate_sizes();
        self.set_natural_size(self.content.get_natural_size());

        let sizing_policy = self.get_styles().sizing_policy.unwrap_or_default();
        self.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height }); 
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.content.set_position(allocated_position);
        self.content.set_size(allocated_size);

        self.content.allocate_space(allocated_position, allocated_size);
    }

    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }
}