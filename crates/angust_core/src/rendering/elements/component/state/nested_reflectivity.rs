use crate::parsing::{
    directive::for_parser::{access_loop_field, ForLoopContext}, 
    html::error::ParsingError
};

use super::reflectivity::ReflectiveState;


pub fn access_field(
    obj: &dyn ReflectiveState,
    field: &str,
    for_loop_contexts: &Option<Vec<ForLoopContext>>,
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
    access_loop_field(field, base_property, nested_property, obj, for_loop_contexts)
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
