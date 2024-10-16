use std::any::Any;

use crate::rendering::elements::component::{component_state::ComponentState, functions::component_functions::ComponentFunctions};

use super::ast::{ASTNode, Operator};

pub fn evaluate_ast<State: ComponentState>(
    node: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    match node {
        ASTNode::Number(num) => Ok(Box::new(*num)),
        ASTNode::Identifier(name) => {
            state.get_property(name)
                .ok_or_else(
                    || format!("Property {} not found in component state", name)
                )
        },
        ASTNode::FunctionCall(name, args) => 
            evaluate_component_function(name, args.clone(), state, functions),
        ASTNode::BinaryOperation { operator, left, right } => 
            evaluate_binary_operation(operator, left, right, state, functions),
        ASTNode::Comparison { operator, left, right } => 
            evaluate_comparison(operator, left, right, state, functions),
        ASTNode::LogicalOperation { operator, left, right } => 
            evaluate_logical_operation(operator, left, right, state, functions),
    }
}

fn evaluate_component_function<State: ComponentState>(
    name: &str,
    args: Vec<ASTNode>,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let arg_values: Result<Vec<Box<dyn Any>>, String> = args.iter()
        .map(|arg| evaluate_ast(arg, state, functions))
        .collect();

    match functions.dynamic_params_functions.get(name) {
        Some(func) => Ok(func(state, arg_values?)),
        None => Err(format!("Function {} not found in component functions", name)),
    }
}

fn evaluate_binary_operation<State: ComponentState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_float = *left_val.downcast::<f64>().map_err(|_| "Type mismatch")?;
    let right_float = *right_val.downcast::<f64>().map_err(|_| "Type mismatch")?;

    let result = match operator {
        Operator::Add => left_float + right_float,
        Operator::Subtract => left_float - right_float,
        Operator::Multiply => left_float * right_float,
        Operator::Divide => left_float / right_float,
        _ => return Err("Unsupported operation for binary operation".to_string()),
    };

    Ok(Box::new(result))
}

fn evaluate_comparison<State: ComponentState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_float = *left_val.downcast::<f64>().map_err(|_| "Type mismatch")?;
    let right_float = *right_val.downcast::<f64>().map_err(|_| "Type mismatch")?;

    let result = match operator {
        Operator::Equal => left_float == right_float,
        Operator::NotEqual => left_float != right_float,
        Operator::Less => left_float < right_float,
        Operator::Greater => left_float > right_float,
        Operator::LessEqual => left_float <= right_float,
        Operator::GreaterEqual => left_float >= right_float,
        _ => return Err("Unsupported operation for comparison".to_string()),
    };

    Ok(Box::new(result))
}

fn evaluate_logical_operation<State: ComponentState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_bool = *left_val.downcast::<bool>().map_err(|_| "Type mismatch")?;
    let right_bool = *right_val.downcast::<bool>().map_err(|_| "Type mismatch")?;

    let result = match operator {
        Operator::And => left_bool && right_bool,
        Operator::Or => left_bool || right_bool,
        _ => return Err("Unsupported operation for logical operation".to_string()),
    };

    Ok(Box::new(result))
}
