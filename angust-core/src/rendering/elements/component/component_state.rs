use std::any::Any;

use crate::parsing::{directive::for_parser::identify_loop_variable_context, html::html_parser::ParsingContext};

use super::reactivity::ComponentEvent;

// Traits expected for any Component State struct
pub trait ReflectiveState {
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>>;
    fn set_field(&mut self, name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
    fn as_any(&self) -> Box<dyn Any>; 
    fn clone_box(&self) -> Box<dyn ReflectiveState>;
}

pub trait ReactiveState : ReflectiveState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

impl Clone for Box<dyn ReflectiveState> {
    fn clone(&self) -> Box<dyn ReflectiveState> {
        self.clone_box()
    }
}

pub fn access_field(
    obj: &dyn ReflectiveState,
    field: &str
) -> Option<Box<dyn ReflectiveState>> {
    let keys: Vec<&str> = field.split('.').collect();

    get_nested_field(obj, &keys)
}

pub fn get_nested_field(
    obj: &dyn ReflectiveState, 
    path: &[&str]
) -> Option<Box<dyn ReflectiveState>> {
    let mut current: Box<dyn ReflectiveState> = obj.clone_box(); 

    for &field in path {
        current = current.get_field(field)?;
    }
    
    Some(current)
}

pub fn access_field_new<State: ReactiveState>(
    obj: &dyn ReflectiveState,
    field: &str,
    context: &ParsingContext<State>,
) -> Result<Box<dyn ReflectiveState>, String> {
    let property_path: Vec<&str> = field.split('.').collect();
    let base_property = match property_path.get(0) { 
        Some(prop) => prop,
        None => return Err("No property found".to_string()),
    };
    let nested_property = property_path.get(1..).unwrap().join(".");

    let property_reflective = get_nested_field(obj, &property_path).ok_or_else(|| {
        format!("Property not found for '{}'", field)
    });
    if !property_reflective.is_err() {
        return property_reflective; //
    }

    let loop_variable_context = identify_loop_variable_context(field, context).ok_or_else(|| {
        format!("Property not found for '{}'", field)
    })?;
    let state = context.component_state.expect("Component state not found");

    let val = match get_nested_field(state, &vec![*base_property]) {
        Some(val) => val,
        None => {
            return Err(format!("No property found for '{}'", loop_variable_context.array_name));
        },
    };

    let current_index = loop_variable_context.current_index;
    let item_as_reflective = val.get_field(&current_index.to_string()).ok_or_else(|| {
        format!("Index {} out of bounds for '{}'", current_index, loop_variable_context.array_name)
    })?;

    Ok(item_as_reflective)
}

// Implementations
pub struct NoState;

impl ReflectiveState for NoState {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
        None
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReactiveState for NoState {
    fn subscribe_to_property<F>(&mut self, _property_name: &str, _callback: F)
    where
        F: 'static + FnMut(&ComponentEvent),
    {
    }
}

impl Clone for NoState {
    fn clone(&self) -> Self {
        NoState {}
    }
}

impl ReflectiveState for String {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
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

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for u32 {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
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

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for usize {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
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

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl ReflectiveState for f64 {
    fn get_field(&self, _name: &str) -> Option<Box<dyn ReflectiveState>> {
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

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}

impl<T> ReflectiveState for Vec<T>
where
    T: ReflectiveState + Clone + 'static,
{
    fn get_field(&self, name: &str) -> Option<Box<dyn ReflectiveState>> {
        if let Ok(index) = name.parse::<usize>() {
            self.get(index).map(|item| item.clone_box())
        } else {
            if name == "len" {
                Some(Box::new(self.len()))
            } else {
                None
            }
        }
    }

    fn set_field(&mut self, _name: &str, _value: Box<dyn Any>) {
        // Do nothing
    }

    fn get_all_properties(&self) -> Vec<&str> {
        vec!["len"]
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn clone_box(&self) -> Box<dyn ReflectiveState> {
        Box::new(self.clone())
    }
}