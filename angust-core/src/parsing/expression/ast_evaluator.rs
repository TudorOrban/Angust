use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use crate::rendering::elements::component::{component_state::{get_nested_field, get_nested_field_new, ReactiveState, ReflectiveState, ReflectiveStateNew}, functions::component_functions::ComponentFunctions};

use super::ast::{ASTNode, Operator};


// TODO: Fix Box leaks with thread_local! storage
pub fn evaluate_ast_new<State: ReflectiveStateNew>(
    node: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    match node {
        ASTNode::Number(num) => {
            Ok(Box::new(*num))
        },
        ASTNode::StringLiteral(string) => {
            Ok(Box::new(string.clone()))
        },
        ASTNode::Identifier(name) => {
            match get_nested_field_new(state, &[name]) {
                Some(val) => {
                    Ok(Box::new(val.as_any()))
                }
                None => Err(format!("Field {} not found", name)),
            }
        },
        _ => Err("Only identifiers are supported for now".to_string()),
    }
}

pub fn evaluate_ast<'a, State: ReactiveState>(
    node: &ASTNode,
    state: &'a State,
    functions: &ComponentFunctions<State>,
) -> Result<&'a dyn Any, String> {
    match node {
        ASTNode::Number(num) => {
            let leaked_value = Box::leak(Box::new(*num) as Box<dyn Any>);
            Ok(leaked_value as &'a dyn Any)
        },
        ASTNode::StringLiteral(string) => {
            let leaked_value = Box::leak(Box::new(string.clone()) as Box<dyn Any>);
            Ok(leaked_value as &'a dyn Any)
        },
        ASTNode::Boolean(boolean) => {
            let leaked_value = Box::leak(Box::new(*boolean) as Box<dyn Any>);
            Ok(leaked_value as &'a dyn Any)
        },
        ASTNode::Identifier(name) => {
            match get_nested_field(state, &[name]) {
                Some(val) => {
                    Ok(val.as_any())
                }
                None => Err(format!("Field {} not found", name)),
            }
        },
        ASTNode::FunctionCall(name, args) => {
            evaluate_component_function(name, args.clone(), state, functions)
        },
        ASTNode::BinaryOperation { operator, left, right } =>
            evaluate_binary_operation(operator, left, right, state, functions),
        ASTNode::Comparison { operator, left, right } =>
            evaluate_comparison(operator, left, right, state, functions),
        ASTNode::LogicalOperation { operator, left, right } =>
            evaluate_logical_operation(operator, left, right, state, functions),
    }
}


fn evaluate_component_function<'a, State: ReactiveState>(
    name: &str,
    args: Vec<ASTNode>,
    state: &'a State,
    functions: &ComponentFunctions<State>,
) -> Result<&'a dyn Any, String> {
    let arg_values: Result<Vec<&dyn Any>, String> = args.iter()
        .map(|arg| evaluate_ast(arg, state, functions))
        .collect();

    match functions.dynamic_params_functions.get(name) {
        Some(func) => Ok(func(state, arg_values?)),
        None => Err(format!("Function {} not found in component functions", name)),
    }
}

fn evaluate_binary_operation<'a, State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &'a State,
    functions: &ComponentFunctions<State>,
) -> Result<&'a dyn Any, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_float = *left_val.downcast_ref::<f64>().ok_or("Type mismatch")?;
    let right_float = *right_val.downcast_ref::<f64>().ok_or("Type mismatch")?;

    let result = match operator {
        Operator::Add => left_float + right_float,
        Operator::Subtract => left_float - right_float,
        Operator::Multiply => left_float * right_float,
        Operator::Divide => left_float / right_float,
        _ => return Err("Unsupported operation for binary operation".to_string()),
    };

    Ok(Box::leak(Box::new(result)) as &'a dyn Any) 
}

fn evaluate_comparison<'a, State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &'a State,
    functions: &ComponentFunctions<State>,
) -> Result<&'a dyn Any, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    if let Some(result) = try_numeric_comparison(operator, left_val, right_val)? {
        return Ok(Box::leak(Box::new(result)) as &dyn Any);  
    }

    if let Some(result) = try_string_comparison(operator, left_val, right_val)? {
        return Ok(Box::leak(Box::new(result)) as &dyn Any);
    }

    Err("Type mismatch or unsupported comparison type".to_string())
}

fn try_numeric_comparison<'a>(
    operator: &Operator,
    left_val: &'a dyn Any,
    right_val: &'a dyn Any,
) -> Result<Option<bool>, String> {
    if let (Some(left_float), Some(right_float)) = (
        left_val.downcast_ref::<f64>(),
        right_val.downcast_ref::<f64>(),
    ) {
        let result = match operator {
            Operator::Equal => left_float == right_float,
            Operator::NotEqual => left_float != right_float,
            Operator::Less => left_float < right_float,
            Operator::Greater => left_float > right_float,
            Operator::LessEqual => left_float <= right_float,
            Operator::GreaterEqual => left_float >= right_float,
            _ => return Err("Unsupported operator for numeric comparison".to_string()),
        };

        return Ok(Some(result));
    }
    Ok(None)
}

fn try_string_comparison<'a>(
    operator: &Operator,
    left_val: &'a dyn Any,
    right_val: &'a dyn Any,
) -> Result<Option<bool>, String> {
    if let (Some(left_str), Some(right_str)) = (
        left_val.downcast_ref::<String>(),
        right_val.downcast_ref::<String>(),
    ) {
        let result = match operator {
            Operator::Equal => left_str == right_str,
            Operator::NotEqual => left_str != right_str,
            _ => return Err("String comparison only supports Equal and NotEqual".to_string()),
        };

        return Ok(Some(result));
    }
    Ok(None)
}


fn evaluate_logical_operation<'a, State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &'a State,
    functions: &ComponentFunctions<State>,
) -> Result<&'a dyn Any, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_bool = *left_val.downcast_ref::<bool>().ok_or("Type mismatch")?;
    let right_bool = *right_val.downcast_ref::<bool>().ok_or("Type mismatch")?;

    let result = match operator {
        Operator::And => left_bool && right_bool,
        Operator::Or => left_bool || right_bool,
        _ => return Err("Unsupported operation for logical operation".to_string()),
    };

    Ok(Box::leak(Box::new(result)) as &'a dyn Any) 
}
