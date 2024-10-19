use std::any::Any;

use super::reactivity::ComponentEvent;

pub enum StateValue<T> {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nested(T),
}

// impl StateValue< {
//     pub fn as_text(&self) -> Option<&String> {
//         if let StateValue::Text(ref val) = self {
//             Some(val)
//         } else {
//             None
//         }
//     }

//     pub fn as_number(&self) -> Option<f64> {
//         if let StateValue::Number(val) = self {
//             Some(*val)
//         } else {
//             None
//         }
//     }

//     // Delegating to the nested state's `get_property`
//     pub fn get_nested_property(&self, property_name: &str) -> Option<StateValue> {
//         if let StateValue::Nested(ref nested) = self {
//             nested.get_property(property_name)
//         } else {
//             None
//         }
//     }
// }



pub trait ComponentState {
    type Output;

    fn get_property(&self, path: &[&str]) -> Option<String>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
}

pub trait ReactiveState : ComponentState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}


// Implementations
pub struct NoState {}

impl ComponentState for NoState {
    type Output = ();

    fn get_property(&self, path: &[&str]) -> Option<String> {
        None
    }

    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
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
    type Output = String;

    fn get_property(&self, path: &[&str]) -> Option<String> {
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
    type Output = f64;

    fn get_property(&self, path: &[&str]) -> Option<String> {
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
    type Output = bool;

    fn get_property(&self, path: &[&str]) -> Option<String> {
        None
    }
    
    fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }
    
    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }
}