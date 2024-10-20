use super::reactivity::ComponentEvent;


// Core traits of the component state, enabling (nested) reflection and reactivity respectively
pub trait Reflect {
    fn get_field(&self, name: &str) -> Option<&dyn Reflect>;
    fn set_field(&mut self, name: &str, value: Box<dyn std::any::Any>);
    fn get_all_properties(&self) -> Vec<&str>;

    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait ReactiveState : Reflect {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

pub fn get_nested_field<'a>(obj: &'a dyn Reflect, path: &[&str]) -> Option<&'a dyn Reflect> {
    let mut current = obj;
    for &field in path {
        if let Some(next) = current.get_field(field) {
            current = next;
        } else {
            return None;
        }
    }
    Some(current)
}

// Implementations
pub struct NoState;

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