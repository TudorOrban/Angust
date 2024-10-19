use std::any::Any;

use super::reactivity::ComponentEvent;

pub enum StateValue<T> {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nested(T),
}

pub trait Reflect {
    fn get_field(&self, name: &str) -> Option<&dyn Reflect>;
    fn set_field(&mut self, name: &str, value: Box<dyn std::any::Any>);
    fn get_all_properties(&self) -> Vec<&str>;

    fn as_any(&self) -> &dyn std::any::Any;
}


impl Reflect for String {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
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

impl Reflect for u32 {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
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

impl Reflect for u8 {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
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

impl Reflect for f64 {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
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

pub trait ComponentState {
    fn get_property(&self, path: &[&str]) -> Option<String>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
}

pub trait ReactiveState : Reflect {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}


// Implementations
pub struct NoState {}

impl ComponentState for NoState {
    fn get_property(&self, _path: &[&str]) -> Option<String> {
        None
    }

    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
}

impl Reflect for NoState {
    fn get_field(&self, _name: &str) -> Option<&dyn Reflect> {
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

impl ComponentState for String {
    fn get_property(&self, _path: &[&str]) -> Option<String> {
        None
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
}

impl ComponentState for f64 {
    fn get_property(&self, _path: &[&str]) -> Option<String> {
        None
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
}

impl ComponentState for bool {
    fn get_property(&self, _path: &[&str]) -> Option<String> {
        None
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
}