use regex::Regex;

use crate::{
    parsing::html::html_parser::ParsingContext, 
    rendering::elements::component::component_state::{access_field, ReactiveState}
};

use super::id_generator::IDGenerator;


// For directive @for="for item in array"
pub fn parse_for_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<ForLoopContext, String> {
    let for_expression = match parse_for_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(ForLoopContext::default()), // No for directive found
    };

    let re = Regex::new(r"let (\w+) of (\w+)").unwrap();
    let captures = match re.captures(&for_expression) {
        Some(captures) => captures,
        None => return Err("Invalid for directive".to_string()),
    };

    let loop_variable = captures.get(1).unwrap().as_str();
    let array_name = captures.get(2).unwrap().as_str();

    // let state = context.component_state.expect("Component state not found");
    // let array_property = state.get_field(array_name).ok_or(
    //     format!("Array '{}' not found in state", array_name)
    // )?;
    // let array_len = match array_property.get_field("len") {
    //     Some(len) => len.as_any().downcast_ref::<usize>().unwrap().clone(),
    //     None => return Err(format!("Array '{}' has no length property", array_name)),
    // };
    Ok(ForLoopContext {
        is_for_loop: true,
        loop_variable: loop_variable.to_string(),
        array_name: array_name.to_string(),
        array_length: 2,
        ..Default::default()
    })
}

pub fn parse_for_attribute<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
) -> Option<String> {
    if let Some(expression_value) = attributes.get("@for") {
        let expression = expression_value.to_string().trim().to_string();
        return Some(expression);
    }
    None
}


// Placeholder replacer for for loop context
pub fn find_property_in_for_loop_variables<State: ReactiveState>(
    property_access_path: &str,
    state: &State,
    context: &ParsingContext<State>,
) -> Result<String, String> {
    if context.for_loop_contexts.is_none() {
        return Err("No for loop contexts found".to_string());
    }
    let for_loop_contexts = context.for_loop_contexts.as_ref().unwrap();

    let property_path: Vec<&str> = property_access_path.split('.').collect();
    let base_property = match property_path.get(0) { 
        Some(prop) => prop,
        None => return Err("Invalid property path".to_string()),
    };
    let nested_property = property_path.get(1..).unwrap().join(".");

    for for_loop_context in for_loop_contexts.iter() {
        if for_loop_context.loop_variable != *base_property {
            continue;
        }

        return find_loop_variable_property(&nested_property, state, for_loop_context);
    }

    return Err("Property not found".to_string());
}

fn find_loop_variable_property<State: ReactiveState>(
    nested_property: &str,
    state: &State,
    for_loop_context: &ForLoopContext,
) -> Result<String, String> {
    let val = match access_field(state, &for_loop_context.array_name) {
        Some(val) => val,
        None => {
            return Err(format!("No property found for '{}'", for_loop_context.array_name));
        },
    };

    let current_index = for_loop_context.current_index;
    let item_as_reflective = val.get_field(&current_index.to_string()).ok_or_else(|| {
        format!("Index {} out of bounds for '{}'", current_index, for_loop_context.array_name)
    })?;

    if nested_property.is_empty() { // Cover case where property is just the loop variable
        let item_as_string = item_as_reflective.as_any().downcast_ref::<String>().ok_or_else(|| {
            format!("Loop variable is not a string")
        })?.clone();

        return Ok(item_as_string);
    }
    
    let nested_val = item_as_reflective.get_field(&nested_property).ok_or_else(|| {
        format!("No property found for '{}'", nested_property)
    })?;

    let nested_val_str = nested_val.as_any().downcast_ref::<String>().ok_or_else(|| {
        format!("Property '{}' is not a string", nested_property)
    })?.clone();

    return Ok(nested_val_str);
}


#[derive(Debug, Clone)]
pub struct ForLoopContext {
    pub context_id: String,
    pub is_for_loop: bool,
    pub loop_variable: String,
    pub array_name: String,
    pub array_length: usize,
    pub current_index: usize,
}

impl Default for ForLoopContext {
    fn default() -> Self {
        Self {
            context_id: IDGenerator::get(),
            is_for_loop: false,
            loop_variable: "".to_string(),
            array_name: "".to_string(),
            array_length: 0,
            current_index: 0,
        }
    }
}