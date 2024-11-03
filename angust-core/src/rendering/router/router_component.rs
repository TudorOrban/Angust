use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{
    application::event_loop_proxy::{get_event_loop_proxy, ApplicationEvent},
    rendering::{
        elements::{
            common_types::{OptionalSize, Position, Size}, 
            component::{component::ComponentInterface, component_factory_registry::create_component}, 
            container::Container, 
            element::{Element, ElementType, EventType}, 
            element_id_generator::ElementIDGenerator, 
            styles::Styles
        }, 
        layout::effective_size_estimator
    }
};

use super::router_proxy::{get_router, RouterProxy};


pub struct RouterComponent {
    _id: String,
    position: Position,
    size: Size, 
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,

    current_component: Box<dyn Element>,
    router: RouterProxy,
}

impl RouterComponent {
    pub fn new() -> Self {
        let id = ElementIDGenerator::get();
        let component = RouterComponent {
            _id: id,
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            current_component: Box::new(Container::new()),
            router: get_router(),
        };

        let callback = component.get_route_change_callback();
        component.router.subscribe_to_current_route(callback); // TODO: Fix this. Right now, template reloading will cause multiple subscriptions

        component
    }

    fn get_route_change_callback(&self) -> Arc<dyn Fn(&str, &str) + Send + Sync + 'static> {
        Arc::new(move |route: &str, component_name: &str| {
            let event_proxy_option = get_event_loop_proxy();
            if event_proxy_option.is_none() {
                println!("Event proxy is None");
                return;
            }
            let event_proxy = event_proxy_option.unwrap();

            event_proxy.send_event(ApplicationEvent::RouteChange(route.to_string(), component_name.to_string()))
                .expect("Failed to send event to GUI thread");
        })
    }

    fn update_current_component(&mut self, component_name: &String, inputs: HashMap<String, Box<dyn Any>>) {
        let component_optional = create_component(component_name);
        if component_optional.is_none() {
            println!("Component not found: {}", component_name);
            return;
        }
        let mut component_box = component_optional.unwrap();
        
        component_box.initialize(inputs);

        self.current_component = component_box;
    }
}

impl Element for RouterComponent {
    
    fn render(&self, canvas: &skia_safe::Canvas) {
        self.current_component.render(canvas);
    }

    fn update(&mut self) {
        self.current_component.update();
    }
    
    fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        self.current_component.handle_event(cursor_position, event_type);
    }
    
    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<String> {
        self.current_component.propagate_event(cursor_position, event_type)
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        self.current_component.add_child(child);
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
        self.current_component.set_styles(styles);
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
    
    fn get_name(&self) -> String {
        self.current_component.get_name()
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
        return self.current_component.get_children_mut();
    }

    // Custom component
    fn get_component_interface(&mut self) -> Option<&mut dyn ComponentInterface> {
        None
    }

    fn initialize(&mut self, inputs: HashMap<String, Box<dyn Any>>) {
        let component_name_opt = self.router.get_current_component_name();
        if component_name_opt.is_none() {
            return;
        }
        let component_name = component_name_opt.unwrap();

        self.update_current_component(&component_name, inputs);
    }

    fn handle_route_change(&mut self, _: &String, component_name: &String) {
        self.update_current_component(component_name, HashMap::new());
    }

    // Layout system
    fn estimate_sizes(&mut self) {
        self.current_component.estimate_sizes();
        self.set_natural_size(self.current_component.get_natural_size());

        let sizing_policy = self.get_styles().sizing_policy.unwrap_or_default();
        self.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height }); 
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.current_component.set_position(allocated_position);
        self.current_component.set_size(allocated_size);

        self.current_component.allocate_space(allocated_position, allocated_size);
    }

    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }

    // Reactivity
    fn react_to_state_change(&mut self, component_id: String) {
        self.current_component.react_to_state_change(component_id);
    }
    
    
}