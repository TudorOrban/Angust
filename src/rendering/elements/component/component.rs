use std::collections::HashMap;

use crate::rendering::elements::element::Element;

pub struct Component<State> {
    pub name: String,
    pub template_relative_path: String,
    pub content: Option<Box<dyn Element>>,
    pub state: State,
    event_handlers: HashMap<String, Box<dyn FnMut(&mut State)>>,
}

impl<State> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        let mut component = Self {
            name,
            template_relative_path: template_relative_path.clone(),
            content: None,
            state,
            event_handlers: HashMap::new(),
        };
        component.initialize(template_relative_path);
        component
    }

    fn initialize(&mut self, template_relative_path: String) {
        // Load template
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