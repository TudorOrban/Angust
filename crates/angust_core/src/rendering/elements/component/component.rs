use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use regex::Regex;

use crate::{
    application::event_loop_proxy::{get_event_loop_proxy, ApplicationEvent}, 
    parsing::{
        directive::for_parser::ForLoopContext, 
        expression::{ast::ASTNode, ast_evaluator}
    }, 
    rendering::{
        elements::{
            button::EventPropagationData, 
            common_types::{OptionalSize, Position, Size}, 
            container::Container, 
            element::{Element, ElementType, EventType}, 
            element_id_generator::ElementIDGenerator, 
            event_propagator, 
            styles::Styles
        }, 
        layout::size_estimation_system::effective_size_estimator,  
    }
};

use super::{
    functions::component_functions::ComponentFunctions, 
    state::reactivity::{EventQueue, ReactiveState}, 
    template_loader
};

pub struct Component<State: ReactiveState> {
    // Component properties
    _id: String,
    pub name: String,
    pub template_relative_path: String,
    pub content: Box<dyn Element>,

    // General properties
    position: Position,
    size: Size, 
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,

    // User-defined properties
    pub state: State,
    pub component_functions: ComponentFunctions<State>,

    // Reactivity
    pub event_queue: Rc<RefCell<EventQueue>>,

    // Expression evaluation
    pub template_expressions_asts: Vec<ASTNode>,
    pub template_event_handler_asts: HashMap<String, ASTNode>,
    pub input_expressions_asts: HashMap<String, ASTNode>,
}

impl<State: ReactiveState> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        let id = ElementIDGenerator::get();
        Self {
            _id: id,
            name,
            template_relative_path,
            content: Box::new(Container::new()),
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            state,
            component_functions: ComponentFunctions::default(),
            event_queue: Rc::new(RefCell::new(EventQueue::new())), 
            template_expressions_asts: vec![],
            template_event_handler_asts: HashMap::new(),
            input_expressions_asts: HashMap::new(),
        }
    }

    // Initialization
    fn initialize(&mut self, inputs: HashMap<String, Box<dyn Any>>) {
        self.setup_listeners();
        self.trigger_user_defined_init();
        self.load_component_template(inputs);
    }

    fn setup_listeners(&mut self) {
        let component_id = self._id.clone();

        let event_proxy_option = get_event_loop_proxy();
        if event_proxy_option.is_none() {
            println!("Event proxy is None");
            return;
        }
        let event_proxy = event_proxy_option.unwrap();
        
        let all_properties: Vec<String> = self.state.get_all_properties().into_iter().map(|s| s.to_string()).collect();
    
        for property_name in all_properties.iter() {
            let property_name_clone = property_name.clone(); 
            let component_id_clone = component_id.clone(); 
            let event_proxy_clone = event_proxy.clone();
            self.state.subscribe_to_property(&property_name_clone, move |event: &ApplicationEvent| {
                match event {
                    ApplicationEvent::StateChange(_) => {
                        event_proxy_clone.send_event(ApplicationEvent::StateChange(component_id_clone.clone()))
                            .expect("Failed to send event");
                    },
                    _ => ()
                }
            });
        }
    }
    
    fn load_component_template(&mut self, inputs: HashMap<String, Box<dyn Any>>) {
        template_loader::load_component_template(self, inputs);
    }

    fn trigger_user_defined_init(&mut self) {
        let user_defined_init_optional = self.component_functions.initialization_function.as_ref();
        if user_defined_init_optional.is_none() {
            return;
        }
        let user_defined_init = user_defined_init_optional.unwrap();
        
        user_defined_init(&mut self.state);
    }

    // Setters
    pub fn add_component_functions(&mut self, functions: ComponentFunctions<State>) {
        self.component_functions = functions;

    }

    pub fn add_event_handler<F>(&mut self, event_name: String, handler: F)
    where
        F: 'static + FnMut(&mut State),
    {
        self.component_functions.event_handlers.insert(event_name, Box::new(handler));
    }
    
    pub fn add_event_handlers(&mut self, handlers: Vec<(&str, Box<dyn FnMut(&mut State)>)>) {
        for (event_name, handler) in handlers {
            self.component_functions.event_handlers.insert(event_name.to_string(), handler);
        }
    }

    // Internal
    // - Event handling
    fn trigger_dynamic_params_event_handler(&mut self, event_name: &String, event_ast: &ASTNode, for_loop_contexts: Vec<ForLoopContext>) {
        let params_asts = match event_ast {
            ASTNode::FunctionCall(_, params) => params,
            _ => return,
        };

        let param_values = self.determine_handler_params(params_asts, for_loop_contexts);

        // Identify the function name (to be refactored later)
        let regex = Regex::new(r"^(.+?)_id_\d+$").unwrap();
        let function_key = regex.captures(event_name)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str());

        if let Some(key) = function_key {
            if let Some(handler) = self.component_functions.dynamic_params_event_handlers.get_mut(key) {
                handler(&mut self.state, param_values);
            } else {
                println!("No handler found for the function name: {}", key);
            }
        } else {
            println!("No function name could be extracted from the event name: {}", event_name);
        }
    }

    fn determine_handler_params(&mut self, params_asts: &Vec<ASTNode>, for_loop_contexts: Vec<ForLoopContext>) -> Vec<Box<dyn Any>> {
        let mut param_values: Vec<Box<dyn Any>> = vec![];

        for params_ast in params_asts {
            let param_value = match ast_evaluator::evaluate_ast(&params_ast, &self.state, &self.component_functions, &for_loop_contexts) {
                Ok(value) => value,
                Err(e) => {
                    println!("Error evaluating dynamic params: {}", e);
                    return vec![];
                }
            };

            param_values.push(param_value);
        }

        param_values
    }

    // - Input handling
    fn update_children_inputs(&mut self) {
        let results = self.evaluate_input_expressions();

        let input_setters = &self.component_functions.input_setters;

        for (property_name, ast_result) in results {
            let setter_name = format!("set_{}", property_name);
            let input_setter_opt = input_setters.get(&setter_name);
            if input_setter_opt.is_none() {
                println!("No input setter found for property: {}", property_name);
                continue;
            }
            let input_setter = input_setter_opt.unwrap();

            input_setter(&mut self.state, vec![ast_result]);
        }
    }

    fn evaluate_input_expressions(&mut self) -> HashMap<String, Box<dyn Any>> {
        let input_setters = &self.component_functions.input_setters;

        let mut results: HashMap<String, Box<dyn Any>> = HashMap::new();
        
        for (property_name, ast) in self.input_expressions_asts.iter() {
            let setter_name = format!("set_{}", property_name);
            let input_setter_opt = input_setters.get(&setter_name);
            if input_setter_opt.is_none() {
                println!("No input setter found for property: {}", property_name);
                continue;
            }

            let value = ast_evaluator::evaluate_ast(&ast, &self.state, &self.component_functions, &vec![]).unwrap(); // TODO: Fix for loop contexts

            results.insert(property_name.clone(), value);
        }

        results
    }
}

impl<State: ReactiveState> Element for Component<State> {
    
    fn render(&self, canvas: &skia_safe::Canvas) {
        self.content.render(canvas);
    }

    fn update(&mut self) {
        self.content.update();
    }
    
    fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) {
        self.content.handle_event(cursor_position, event_type);
    }
    
    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<EventPropagationData> {
        let event_propagation_datas = event_propagator::propagate_event(self, cursor_position, event_type);
        
        for data in event_propagation_datas.iter() {
            let handler_name = &data.handler_name;
            let for_loop_contexts = &data.for_loop_contexts;

            if let Some(handler) = self.component_functions.event_handlers.get_mut(handler_name) {
                handler(&mut self.state);
            }
            
            if let Some(event_ast) = self.template_event_handler_asts.get(handler_name) {
                self.trigger_dynamic_params_event_handler(&handler_name.clone(), &event_ast.clone(), for_loop_contexts.clone()); // TODO: Fix this clone
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
    
    fn get_name(&self) -> String {
        self.name.clone()
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

    // Custom component
    fn get_component_interface(&mut self) -> Option<&mut dyn ComponentInterface> {
        Some(self)
    }

    fn initialize(&mut self, inputs: HashMap<String, Box<dyn Any>>) {
        self.initialize(inputs);
    }
    
    fn handle_route_change(&mut self, route: &String, component_name: &String) {
        self.content.handle_route_change(route, component_name);
    }

    // Layout system
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

    // Reactivity
    fn react_to_state_change(&mut self, component_id: String) {
        if component_id == self.get_id() {
            self.load_component_template(HashMap::new()); // Naive approach; to be replaced later
            self.update_children_inputs();
        }
    }
}

pub trait ComponentInterface {
    fn update_input(&mut self, input_name: &str, value: Vec<Box<dyn Any>>);
    fn get_input_asts(&self) -> HashMap<String, ASTNode>;
}

impl<State: ReactiveState> ComponentInterface for Component<State> {
    fn update_input(&mut self, input_name: &str, value: Vec<Box<dyn Any>>) {
        let setter_name = format!("set_{}", input_name);
        if let Some(setter) = self.component_functions.input_setters.get(&setter_name) {
            setter(&mut self.state, value);
        }
    }

    fn get_input_asts(&self) -> HashMap<String, ASTNode> {
        self.input_expressions_asts.clone()
    }
}