use regex::Regex;

use crate::{
    parsing::{
        expression::{ast::{self, ASTNode}, ast_evaluator}, 
        html::id_generator::IDGenerator
    }, 
    rendering::elements::component::component_state::{
        access_field, ReactiveState, ReflectiveState
    }
};

use super::html_parser::ParsingContext;

// State placeholders {{ component_state_property }}
pub fn parse_state_placeholder<State: ReactiveState>(
    text: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, String> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = text.to_string();

    for cap in re.captures_iter(text) {
        let property_path = match cap.get(1) {
            Some(m) => m.as_str().trim(),
            None => continue,
        };

        result = substitute_state_placeholder(property_path, state, context)?;
    }

    Ok(result)
}

pub fn substitute_state_placeholder<State: ReactiveState>(
    property_path: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, String> {
    let mut result = match access_field(state, property_path) {
        Some(val) => {
            if let Some(val) = val.as_any().downcast_ref::<String>() {
                Ok(val.clone())
            } else {
                Err(format!("Property '{}' is not a string", property_path))
            }
        },
        None => {
            Err(format!("No property found for '{}'", property_path))
        },
    };
    if !result.is_err() {
        return result;
    }

    result = find_property_in_for_loop(property_path, state, context);
    if result.is_err() {
        println!("Error: {}", result.as_ref().unwrap());
    }

    result
}

pub fn find_property_in_for_loop<State: ReactiveState>(
    property_path: &str,
    state: &State,
    context: &ParsingContext<State>,
) -> Result<String, String> {
    let result = Err("Property not found".to_string());
    let mut nested_properties: Vec<&str> = property_path.split('.').collect();

    if context.for_loop_contexts.is_none() {
        return result;
    }
    let for_loop_contexts = context.for_loop_contexts.as_ref().unwrap();

    for for_loop_context in for_loop_contexts.iter() {
        if for_loop_context.loop_variable != nested_properties[0] {
            continue;
        }
        

        let val = match access_field(state, &for_loop_context.array_name) {
            Some(val) => val,
            None => {
                return Err(format!("No property found for '{}'", for_loop_context.array_name));
            },
        };

        let val_as_any = val.as_any();
        let reflective_array = if let Some(array) = val_as_any.downcast_ref::<Vec<&dyn ReflectiveState>>() {
            array
        } else {
            return Err(format!("Property '{}' is not an array", for_loop_context.array_name));
        };

        let current_index = 0; // To be replaced with actual index later
        let current_item = reflective_array[current_index];

        let nested_property = nested_properties.pop().unwrap();
        let nested_val = match access_field(current_item, nested_property) {
            Some(val) => val,
            None => {
                return Err(format!("No property found for '{}'", nested_property));
            },
        };

        let nested_val_str = if let Some(val) = nested_val.as_any().downcast_ref::<String>() {
            val.clone()
        } else {
            return Err(format!("Property '{}' is not a string", nested_property));
        };

        return Ok(nested_val_str);
    }

    result
}

// If directive @if="expression"
pub fn parse_if_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<bool, String> {
    let if_expression = match parse_if_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(true), 
    };

    let ast = ast::parse_string_to_ast(if_expression)
        .map_err(|e| format!("Error parsing if expression: {:?}", e))?;
    ParsingContext::add_template_expression_ast(context, ast.clone());

    let state = context.component_state.expect("Component state not found");
    let functions = context.component_functions.expect("Component functions not found");
    let evaluation_result = ast_evaluator::evaluate_ast::<State>(&ast, state, functions)
        .map_err(|e| format!("Error evaluating if expression: {:?}", e))?;

    let is_if_true = evaluation_result
        .downcast_ref::<bool>()
        .ok_or_else(|| format!("If expression did not evaluate to a boolean"))?;

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

// On click attribute @onclick="event_handler_name()"
pub fn parse_on_click_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<(String, ASTNode), String> {
    let on_click_attribute = match parse_on_click_attribute::<State>(attributes, context) {
        Some(expr) => expr,
        None => return Err("No on click attribute found".to_string()),
    };

    let ast = ast::parse_string_to_ast(on_click_attribute.clone())
        .map_err(|e| format!("Error parsing on click expression: {:?}", e))?;

    // Get root function name
    let mut root_function_name = match ast.clone() {
        ASTNode::FunctionCall(function_name, _) => function_name,
        _ => return Err("Invalid on click expression".to_string()),
    };
    let unique_id = IDGenerator::get();
    root_function_name = format!("{}_{}", root_function_name, unique_id);

    Ok((root_function_name, ast))
}

pub fn parse_on_click_attribute<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    _: &ParsingContext<State>
) -> Option<String> {
    if let Some(on_click_value) = attributes.get("@onclick") {
        let handler = on_click_value.to_string();
        let handler = handler.to_string();
        return Some(handler);
    }
    None
}


// For directive @for="for item in array"
#[allow(dead_code)]
pub fn parse_for_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<ForLoopContext, String> {
    let for_expression = match parse_for_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(ForLoopContext::default()),
    };

    let re = Regex::new(r"let (\w+) of (\w+)").unwrap();
    let captures = match re.captures(&for_expression) {
        Some(captures) => captures,
        None => return Err("Invalid for directive".to_string()),
    };

    let loop_variable = captures.get(1).unwrap().as_str();
    let array_name = captures.get(2).unwrap().as_str();

    let state = context.component_state.expect("Component state not found");
    let array_property = match state.get_field(array_name) {
        Some(prop) => prop.as_any(),
        None => return Err(format!("Array '{}' not found in state", array_name)),
    };

    let array = array_property.downcast_ref::<Vec<String>>()
        .ok_or_else(|| format!("Property '{}' is not an array", array_name))?;

    Ok(ForLoopContext {
        is_for_loop: true,
        loop_variable: loop_variable.to_string(),
        array_name: array_name.to_string(),
        array_length: array.len(),
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
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