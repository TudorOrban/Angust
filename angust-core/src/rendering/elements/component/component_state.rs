use std::{any::Any, rc::Rc};

use super::reactivity::ComponentEvent;

// New
pub trait ReflectiveStateNew {
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveStateNew>>;
    fn set_field(&mut self, name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
    fn as_any(&self) -> Box<dyn Any>; 
    fn clone_box(&self) -> Box<dyn ReflectiveStateNew>;
}

impl Clone for Box<dyn ReflectiveStateNew> {
    fn clone(&self) -> Box<dyn ReflectiveStateNew> {
        self.clone_box()
    }
}

pub fn get_nested_field_new(
    obj: &dyn ReflectiveStateNew, 
    path: &[&str]
) -> Option<Box<dyn ReflectiveStateNew>> {
    let mut current: Box<dyn ReflectiveStateNew> = obj.clone_box(); 

    for &field in path {
        current = current.get_field(field)?;
    }
    Some(current)
}



impl ReflectiveStateNew for String {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveStateNew>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveStateNew> {
        Box::new(self.clone())
    }
}




// Old
// Core traits of the component state, enabling (nested) reflection and reactivity respectively
pub trait ReflectiveState {
    fn get_field(&self, name: &str) -> Option<&dyn ReflectiveState>;
    fn set_field(&mut self, name: &str, value: Box<dyn Any>);
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