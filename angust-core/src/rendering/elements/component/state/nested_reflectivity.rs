use crate::parsing::{directive::for_parser::access_loop_field, html::{error::ParsingError, html_parser::ParsingContext}};

use super::{reactivity::ReactiveState, reflectivity::ReflectiveState};


pub fn access_field<State: ReactiveState>(
    obj: &dyn ReflectiveState,
    field: &str,
    context: &ParsingContext<State>,
) -> Result<Box<dyn ReflectiveState>, ParsingError> {
    let property_path: Vec<&str> = field.split('.').collect();
    let base_property = match property_path.get(0) { 
        Some(prop) => prop,
        None => return Err(ParsingError::FieldAccessError("No property found".to_string())),
    };
    let nested_property = property_path.get(1..);

    // Check direct property access firstly
    let property_reflective = get_nested_field(obj, &property_path).ok_or_else(|| {
        ParsingError::FieldAccessError(format!("Property not found for '{}'", field))
    });
    if !property_reflective.is_err() {
        return property_reflective;
    }

    // Check for loop variable secondly
    access_loop_field(context, field, base_property, nested_property)
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
