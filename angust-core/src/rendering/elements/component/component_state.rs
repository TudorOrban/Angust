use std::any::Any;

use super::reactivity::ComponentEvent;


pub trait ComponentState: AsAny {
    fn get_property(&self, property_name: &str) -> Option<Box<dyn Any>>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;

    fn get_nested_state(&self, property_name: &str) -> Option<&dyn ComponentState>;
}

pub trait ReactiveState : ComponentState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


// Implementations
pub struct NoState {}

impl ComponentState for NoState {
    fn get_property(&self, _property_name: &str) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn set_property(&mut self, _property_name: &str, _value: Box<dyn std::any::Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn get_nested_state(&self, _property_name: &str) -> Option<&dyn ComponentState> {
        None
    }
}

impl ReactiveState for NoState {

    fn subscribe_to_property<F>(&mut self, _property_name: &str, _callback: F)
    where
        F: 'static + FnMut(&crate::rendering::elements::component::reactivity::ComponentEvent),
    {
    }
}

impl ComponentState for String {
    fn get_property(&self, _property_name: &str) -> Option<Box<dyn Any>> {
        None // Strings don't have nested properties
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
    
    fn get_nested_state(&self, _property_name: &str) -> Option<&dyn ComponentState> {
        None // Strings aren't complex types
    }
}

impl ComponentState for f64 {
    fn get_property(&self, _property_name: &str) -> Option<Box<dyn Any>> {
        None // f64 doesn't have nested properties
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
    
    fn get_nested_state(&self, _property_name: &str) -> Option<&dyn ComponentState> {
        None // f64 isn't a complex type
    }
}