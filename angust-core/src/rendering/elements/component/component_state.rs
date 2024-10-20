use std::any::Any;

use super::reactivity::ComponentEvent;


// Core traits of the component state, enabling (nested) reflection and reactivity respectively
pub trait ReflectiveState {
    fn get_field(&self, name: &str) -> Option<&dyn ReflectiveState>;
    fn set_field(&mut self, name: &str, value: Box<dyn std::any::Any>);
    fn get_all_properties(&self) -> Vec<&str>;
    fn as_any(&self) -> &dyn Any;
}

pub trait ReactiveState : ReflectiveState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

// Nested reflection
pub fn get_nested_field<'a>(
    obj: &'a dyn ReflectiveState, 
    path: &[&str]
) -> Option<&'a dyn ReflectiveState> {
    let mut current = obj;
    for &field in path {
        current = current.get_field(field)?;
    }
    Some(current)
}


pub struct NoState;

impl ReflectiveState for NoState {
    fn get_field(&self, _name: &str) -> Option<&dyn ReflectiveState> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn std::any::Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ReactiveState for NoState {

    fn subscribe_to_property<F>(&mut self, _property_name: &str, _callback: F)
    where
        F: 'static + FnMut(&crate::rendering::elements::component::reactivity::ComponentEvent),
    {
    }
}

impl ReflectiveState for String {
    fn get_field(&self, _name: &str) -> Option<&dyn ReflectiveState> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn std::any::Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ReflectiveState for u32 {
    fn get_field(&self, _name: &str) -> Option<&dyn ReflectiveState> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn std::any::Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ReflectiveState for u8 {
    fn get_field(&self, _name: &str) -> Option<&dyn ReflectiveState> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn std::any::Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ReflectiveState for f64 {
    fn get_field(&self, _name: &str) -> Option<&dyn ReflectiveState> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn std::any::Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}