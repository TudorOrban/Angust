use std::{any::Any, collections::HashMap};


pub struct ComponentFunctions<State> {
    pub event_handlers: HashMap<String, Box<dyn FnMut(&mut State)>>,
    pub boolean_evaluators: HashMap<String, Box<dyn Fn(&State) -> bool>>,
    pub array_getters: HashMap<String, Box<dyn Fn(&State) -> Vec<&dyn Any>>>,
    pub dynamic_params_functions: HashMap<String, Box<dyn Fn(&State, Vec<Box<dyn Any>>) -> Box<dyn Any>>>,
}

impl<State> ComponentFunctions<State> {
    pub fn new(
        event_handlers: Vec<(&str, Box<dyn FnMut(&mut State)>)>,
        boolean_evaluators: Vec<(&str, Box<dyn Fn(&State) -> bool>)>,
        array_getters: Vec<(&str, Box<dyn Fn(&State) -> Vec<&dyn Any>>)>,
        dynamic_params_functions: Vec<(&str, Box<dyn Fn(&State, Vec<Box<dyn Any>>) -> Box<dyn Any>>)>,
    ) -> Self {
        let mut functions = Self::default();

        for (event_name, handler) in event_handlers {
            functions.event_handlers.insert(event_name.to_string(), handler);
        }
        for (evaluator_name, evaluator) in boolean_evaluators {
            functions.boolean_evaluators.insert(evaluator_name.to_string(), evaluator);
        }
        for (getter_name, getter) in array_getters {
            functions.array_getters.insert(getter_name.to_string(), getter);
        }
        for (function_name, function) in dynamic_params_functions {
            functions.dynamic_params_functions.insert(function_name.to_string(), function);
        }

        functions
    }
}

impl<State> Default for ComponentFunctions<State> {
    fn default() -> Self {
        Self {
            event_handlers: HashMap::new(),
            boolean_evaluators: HashMap::new(),
            array_getters: HashMap::new(),
            dynamic_params_functions: HashMap::new(),
        }
    }
}