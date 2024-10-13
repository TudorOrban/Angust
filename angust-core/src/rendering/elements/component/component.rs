use std::{collections::HashMap, path::PathBuf};

use crate::{application::{angust_configuration::AngustConfiguration, resource_loader::path_navigator::identify_project_root_path}, parsing::{css::stylesheet_parser::Stylesheet, html::html_parser}, rendering::elements::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::Styles}};

pub struct Component<State> {
    _id: String,
    pub name: String,
    pub template_relative_path: String,
    content: Option<Box<dyn Element>>,
    position: Position,
    size: Size,
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,

    // User-defined state
    pub state: State,
    event_handlers: HashMap<String, Box<dyn FnMut(&mut State)>>,
}

impl<State> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        println!("Creating component {}", name);
        let id = IDGenerator::get();
        let mut component = Self {
            _id: id,
            name,
            template_relative_path: template_relative_path,
            content: None,
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
        self.load_template();
    }

    fn load_template(&mut self) {
        // Load template
        println!("Loading template from {}", self.template_relative_path);

        let project_root = PathBuf::from(identify_project_root_path());
        let template_path = project_root.join(&self.template_relative_path);

        println!("Template path: {:?}", template_path);
        let template_content = std::fs::read_to_string(template_path)
            .expect("Failed to read template file");

        // Parse template
        let dom = html_parser::parse_html_content(&template_content);
        println!("Parsed template");
        self.content = html_parser::map_dom_to_elements(&dom, None, &AngustConfiguration::default(), &Stylesheet::default());
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
        if let Some(content) = &self.content {
            content.render(canvas);
        }
    }

    fn update(&mut self) {
        if let Some(content) = &mut self.content {
            content.update();
        }
    }
    
    fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        if let Some(content) = &mut self.content {
            content.handle_event(cursor_position, event_type);
        }
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
        if let Some(content) = &mut self.content {
            content.estimate_sizes();
        }
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        if let Some(content) = &mut self.content {
            content.set_position(allocated_position);
            content.set_size(allocated_size);

            content.allocate_space(allocated_position, allocated_size);
        }
    }

    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }
}