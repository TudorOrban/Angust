use regex::Regex;

use crate::{
    parsing::html::{error::ParsingError, html_parser::ParsingContext}, rendering::elements::component::state::{nested_reflectivity::{access_field, get_nested_field}, reactivity::ReactiveState, reflectivity::ReflectiveState}, 
};

use super::id_generator::IDGenerator;


// For directive @for="for item in array"
pub fn parse_for_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<ForLoopContext, ParsingError> {
    let for_expression = match parse_for_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(ForLoopContext::default()), // No for directive found
    };

    let re = Regex::new(r"let (\w+) of ([\w\.]+)").unwrap();
    let captures = match re.captures(&for_expression) {
        Some(captures) => captures,
        None => return Err(ParsingError::InvalidDirectiveSyntax("Invalid @for directive".to_string())),
    };

    let loop_variable = captures.get(1).unwrap().as_str();
    let array_path = captures.get(2).unwrap().as_str();

    let state = context.component_state.expect("Component state not found");
    let array_property = access_field(state, array_path, &context.for_loop_contexts)?;

    let array_len = match array_property.get_field("len") {
        Some(len) => len.as_any().downcast_ref::<usize>().unwrap().clone(),
        None => return Err(ParsingError::InvalidDirectiveSyntax(format!("Array '{}' has no length property", array_path))),
    };

    Ok(ForLoopContext {
        is_for_loop: true,
        loop_variable: loop_variable.to_string(),
        array_access_path: array_path.to_string(),
        array_length: array_len,
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

// For accessing logic
pub fn access_loop_field(
    field: &str,
    base_property: &str,
    nested_property: Option<&[&str]>,
    state: &dyn ReflectiveState,
    for_loop_contexts: &Option<Vec<ForLoopContext>>,
) -> Result<Box<dyn ReflectiveState>, ParsingError> {
    let loop_variable_context = identify_loop_variable_context(base_property, for_loop_contexts).ok_or_else(|| {
        println!("Loop variable not found for '{}'", base_property);
        ParsingError::FieldAccessError(field.to_string())
    })?;
    let array_access_path = loop_variable_context.array_access_path.clone();

    // Get current loop array
    let array_reflective = access_field(state, &array_access_path, for_loop_contexts).or_else(|_| {
        println!("Array not found for '{}'", array_access_path);
        Err(ParsingError::FieldAccessError(format!("Array not found for '{}'", array_access_path)))
    })?;

    // Get current array item
    let current_index = loop_variable_context.current_index;
    let array_item_as_reflective = array_reflective.get_field(&current_index.to_string()).ok_or_else(|| {
        println!("Index {} out of bounds for '{}'", current_index, array_access_path);
        ParsingError::FieldAccessError(format!("Index {} out of bounds for '{}'", current_index, array_access_path))
    })?;

    if nested_property.is_some() {
        return get_nested_field(&*array_item_as_reflective, &nested_property.unwrap()).ok_or_else(|| {
            ParsingError::FieldAccessError(format!("Test: {:?}", field.to_string()))
        });
    } else {
        return Ok(array_item_as_reflective);
    }
}

pub fn identify_loop_variable_context(
    base_property: &str,
    for_loop_contexts: &Option<Vec<ForLoopContext>>,
) -> Option<ForLoopContext> {
    if for_loop_contexts.is_none() {
        return None;
    }
    let for_loop_contexts = for_loop_contexts.as_ref().unwrap();

    // Functional style
    for_loop_contexts.iter().find(|for_loop_context| for_loop_context.loop_variable == base_property).cloned()
}

#[derive(Debug, Clone)]
pub struct ForLoopContext {
    pub context_id: String,
    pub is_for_loop: bool,
    pub loop_variable: String,
    pub array_access_path: String,
    pub array_length: usize,
    pub current_index: usize,
}

impl Default for ForLoopContext {
    fn default() -> Self {
        Self {
            context_id: IDGenerator::get(),
            is_for_loop: false,
            loop_variable: "".to_string(),
            array_access_path: "".to_string(),
            array_length: 0,
            current_index: 0,
        }
    }
}