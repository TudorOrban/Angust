use std::{any::Any, collections::HashMap};

use regex::Regex;

use crate::{
    parsing::{expression::{ast, ast_evaluator}, html::{error::ParsingError, html_parser::ParsingContext}}, 
    rendering::elements::component::state::reactivity::ReactiveState
};


pub fn parse_input_expressions<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<HashMap<String, Box<dyn Any>>, ParsingError> {
    let inputs = parse_input_attributes(attributes);
    let mut outputs: HashMap<String, Box<dyn Any>> = HashMap::new();

    let state = context.component_state.unwrap();
    let component_functions = context.component_functions.unwrap();

    for (property_name, bound_value) in inputs.iter() {
        let ast = ast::parse_string_to_ast(bound_value.to_string())
            .map_err(|e| ParsingError::ASTParsingError(format!("{:?}", e)))?;

        println!("Input AST: {:?}", ast);
        
        let value = ast_evaluator::evaluate_ast(&ast, state, component_functions)
            .map_err(|e| ParsingError::ASTEvaluationError(format!("{:?}", e)))?;

        outputs.insert(property_name.to_string(), value);
    }

    Ok(outputs)
}

pub fn parse_input_attributes(
    attributes: &kuchiki::Attributes,
) -> HashMap<String, String> {
    let mut inputs = HashMap::new();
    let re = Regex::new(r"\[\s*(\w+)\s*\]").unwrap(); // Regex to capture [property]

    for (key, value) in attributes.map.iter() {
        let key_local = key.local.clone().to_string();
        let optional_capture = re.captures(&key_local);
        if optional_capture.is_none() {
            continue;
        }

        let captures = optional_capture.unwrap().get(1);
        if captures.is_none() {
            continue;
        }

        let property_name = captures.unwrap().as_str();
        let bound_value = value.value.to_string();

        inputs.insert(property_name.to_string(), bound_value);
    }

    inputs
}