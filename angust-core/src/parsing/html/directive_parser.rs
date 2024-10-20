use regex::Regex;

use crate::{parsing::expression::{ast, ast_evaluator}, rendering::elements::component::component_state::{get_nested_field, ReactiveState, ReflectiveState}};

use super::html_parser::ParsingContext;

// On click attribute @onclick="event_handler_name()"
pub fn parse_on_click_attribute<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    _: &ParsingContext<State>
) -> Option<String> {
    if let Some(on_click_value) = attributes.get("@onclick") {
        let handler = on_click_value.to_string();
        let handler = handler.trim_start_matches("('");
        let handler = handler.trim_end_matches("')");
        let handler = handler.to_string();
        return Some(handler);
    }
    None
}

// State placeholders {{ component_state_property }}

pub fn parse_state_placeholder<State: ReflectiveState>(
    text: &str,
    state: &State,
) -> Result<String, String> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = text.to_string();

    for cap in re.captures_iter(text) {
        // Extract the full matched text as a string slice
        let full_match = match cap.get(0) {
            Some(m) => m.as_str(),
            None => continue,
        };

        // Extract the property path, trimming spaces
        let property_path = match cap.get(1) {
            Some(m) => m.as_str().trim(),
            None => continue,
        };

        // Split the property path into parts for nested access
        let keys: Vec<&str> = property_path.split('.').collect();
        match get_nested_field(state, keys.as_slice()) {
            Some(val) => {
                if let Some(val) = val.as_any().downcast_ref::<String>() {
                    println!("String value: {}", val);
                    result = result.replace(full_match, val);
                }
            },
            None => {
                return Err(format!("No property found for '{}'", full_match));
            },
        }
    }

    Ok(result)
}



// If directive @if="expression"
pub fn parse_if_expression<State: ReactiveState>(
    context: &mut ParsingContext<State>,
    attributes: &kuchiki::Attributes,
) -> Result<bool, String> {
    let if_expression = match parse_if_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(true), 
    };

    let ast = ast::parse_string_to_ast(if_expression).map_err(|e| format!("Error parsing if expression: {:?}", e))?;
    ParsingContext::add_ast(context, ast.clone());

    let state = context.component_state.expect("Component state not found");
    let functions = context.component_functions.expect("Component functions not found");
    let evaluation_result = ast_evaluator::evaluate_ast::<State>(&ast, state, functions)
        .map_err(|e| format!("Error evaluating if expression: {:?}", e))?;

    let is_if_true = evaluation_result
        .downcast_ref::<bool>()
        .ok_or_else(|| "If expression did not evaluate to a boolean".to_string())?;

    println!("If expression evaluated to: {}", is_if_true);
    Ok(*is_if_true)
}

pub fn parse_if_attribute<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
) -> Option<String> {
    if let Some(expression_value) = attributes.get("@if") {
        let expression = expression_value.to_string().trim().to_string();
        return Some(expression);
    }
    None
}

// For directive @for="for item in array"
// pub fn parse_for_expression<State: ReactiveState>(
//     context: &mut ParsingContext<State>,
//     attributes: &kuchiki::Attributes,
// ) -> Result<ForLoopContext, String> {
//     let for_expression = match parse_for_attribute::<State>(attributes) {
//         Some(expr) => expr,
//         None => return Ok(ForLoopContext::default()),
//     };

//     let re = Regex::new(r"let (\w+) of (\w+)").unwrap();
//     let captures = match re.captures(&for_expression) {
//         Some(captures) => captures,
//         None => return Err("Invalid for directive".to_string()),
//     };

//     let loop_variable = captures.get(1).unwrap().as_str();
//     let array_name = captures.get(2).unwrap().as_str();

//     let state = context.component_state.as_ref().expect("Component state not found");
//     let array_property = match state.get_field(array_name) {
//         Some(prop) => prop.as_any(),
//         None => return Err(format!("Array '{}' not found in state", array_name)),
//     };

//     let array = array_property.downcast_ref::<Vec<String>>()
//         .ok_or_else(|| format!("Property '{}' is not an array", array_name))?;

//     Ok(ForLoopContext {
//         is_for_loop: true,
//         loop_variable: loop_variable.to_string(),
//         array_name: array_name.to_string(),
//         array_length: array.len(),
//     })
// }

// pub fn parse_for_attribute<State: ReactiveState>(
//     attributes: &kuchiki::Attributes,
// ) -> Option<String> {
//     if let Some(expression_value) = attributes.get("@for") {
//         let expression = expression_value.to_string().trim().to_string();
//         return Some(expression);
//     }
//     None
// }

#[derive(Debug)]
pub struct ForLoopContext {
    pub is_for_loop: bool,
    pub loop_variable: String,
    pub array_name: String,
    pub array_length: usize,
}

impl Default for ForLoopContext {
    fn default() -> Self {
        Self {
            is_for_loop: false,
            loop_variable: "".to_string(),
            array_name: "".to_string(),
            array_length: 0,
        }
    }
}