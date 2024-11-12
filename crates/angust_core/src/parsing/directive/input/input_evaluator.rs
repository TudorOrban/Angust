use std::{any::Any, collections::HashMap};

use crate::{
    parsing::{expression::ast_evaluator, html::{error::ParsingError, html_parser::ParsingContext}}, 
    rendering::elements::{component::state::reactivity::ReactiveState, element::Element}
};


pub fn compute_inputs_from_parent_component<State: ReactiveState>(
    component_box: &Box<dyn Element>, 
    context: &ParsingContext<State>,
) -> Result<HashMap<String, Box<dyn Any>>, ParsingError> {
    let state = match context.component_state {
        Some(state) => state,
        None => return Ok(HashMap::new()), // Root component, no parent component
    };
    let component_functions = match context.component_functions {
        Some(functions) => functions,
        None => return Ok(HashMap::new()),
    };
    let scanned_inputs = match context.scanned_inputs.clone() {
        Some(inputs) => inputs,
        None => return Ok(HashMap::new()),
    };

    let mut input_values: HashMap<String, Box<dyn Any>> = HashMap::new(); // component_id -> input_value

    for ((component_name, input_name), input_ast) in scanned_inputs.iter() {
        if *component_name != component_box.get_name() { // TODO: Fix this: account for multiple components with same name
            continue;
        }

        let input_value = ast_evaluator::evaluate_ast(input_ast, state, component_functions, &context.for_loop_contexts.clone().unwrap_or(vec![]))?;

        input_values.insert(input_name.clone(), input_value);
    }

    Ok(input_values)
}