use std::{collections::HashMap, path::PathBuf};

use crate::{application::{angust_configuration::AngustConfiguration, resource_loader::path_navigator::identify_project_root_path}, parsing::{css::stylesheet_parser::Stylesheet, html::html_parser}, rendering::elements::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::Styles}};

use super::template_loader;

pub struct Component<State> {
    _id: String,
    pub name: String,
    pub template_relative_path: String,
    pub content: Option<Box<dyn Element>>,
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
        template_loader::load_template(self);
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
        if let Some(content) = &mut self.content {
            content.add_child(child);
        }
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
        let effective_width = if let Some(width) = self.get_requested_size().width {
            width.value
        } else {
            self.get_natural_size().width
        };
        let effective_height = if let Some(height) = self.get_requested_size().height {
            height.value
        } else {
            self.get_natural_size().height
        };

        Size {
            width: effective_width,
            height: effective_height,
        }
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }
    
    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        if let Some(content) = &mut self.content {
            println!("Getting children for component: {}", self.name);
            return content.get_children_mut();
        }
        None
    }

    fn estimate_sizes(&mut self) {
        let mut content_natural_size = Size::default();
        if let Some(content) = &mut self.content {
            content.estimate_sizes();
            content_natural_size = content.get_natural_size();
        }
        self.set_natural_size(content_natural_size);

        let sizing_policy = self.get_styles().sizing_policy.unwrap_or_default();
        self.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height }); 
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