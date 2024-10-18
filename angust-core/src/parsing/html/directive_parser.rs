use regex::Regex;

use crate::{parsing::expression::{ast, ast_evaluator}, rendering::elements::component::component_state::ComponentState};

use super::html_parser::ParsingContext;


pub fn parse_on_click_attribute<State: ComponentState>(
    attributes: &kuchiki::Attributes,
    _: &ParsingContext<State>
) -> Option<String> {
    if let Some(on_click_value) = attributes.get("@onclick") {
        let handler = on_click_value.to_string();
        let handler = handler.trim_start_matches("handle_event('");
        let handler = handler.trim_end_matches("')");
        let handler = handler.to_string();
        return Some(handler);
    }
    None
}

pub fn parse_if_attribute<State: ComponentState>(
    attributes: &kuchiki::Attributes,
) -> Option<String> {
    if let Some(expression_value) = attributes.get("@if") {
        let expression = expression_value.to_string().trim().to_string();
        return Some(expression);
    }
    None
}

pub fn parse_state_placeholder<State: ComponentState>(
    text: &str,
    state: &State,
) -> Result<String, String> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = text.to_string();

    for cap in re.captures_iter(text) {
        let matched_text = match cap.get(0) {
            Some(text) => text,
            None => continue,
        };

        let key = cap[1].trim();
        let property = match state.get_property(key) {
            Some(prop) => prop,
            None => return Err(format!("Property '{}' not found in state", key)),
        };

        let value = if let Some(val) = property.downcast_ref::<String>() {
            val.clone()
        } else if let Some(val) = property.downcast_ref::<f64>() {
            val.to_string()
        } else if let Some(val) = property.downcast_ref::<i32>() {
            val.to_string()
        } else {
            return Err(format!("Property '{}' is of an unsupported type", key));
        };

        result = result.replace(matched_text.as_str(), &value);
    }

    Ok(result)
}

pub fn parse_if_expression<State: ComponentState>(
    context: &mut ParsingContext<State>,
    attributes: &kuchiki::Attributes,
) -> Result<bool, String> {
    let if_expression_option = parse_if_attribute::<State>(&attributes);
    if let Some(if_expression) = if_expression_option {
        let ast_result = ast::parse_string_to_ast(if_expression);
        if let Ok(ast) = ast_result {
            ParsingContext::add_ast(context, ast.clone());

            let state = context.component_state.unwrap();
            let functions = context.component_functions.unwrap();
            let evaluation_result = ast_evaluator::evaluate_ast::<State>(&ast, state, functions);
            
            if let Ok(result) = evaluation_result {
                if let Some(is_if_true) = result.downcast_ref::<bool>() {
                    println!("If expression evaluated to: {}", is_if_true);
                    return Ok(*is_if_true);
                } else {
                    return Err("If expression did not evaluate to a boolean".to_string());
                }
            } else {
                return Err("Failed to evaluate if expression".to_string());
            }
        } else {
            return Err("Failed to parse if expression".to_string());
        }
    } else {
        return Ok(true);
    }
}