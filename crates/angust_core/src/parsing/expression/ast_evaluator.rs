use std::any::Any;

use crate::{
    parsing::{
        directive::for_parser::ForLoopContext, 
        html::error::ParsingError
    }, 
    rendering::elements::component::{
        functions::component_functions::ComponentFunctions, 
        state::{nested_reflectivity::access_field, reactivity::ReactiveState}
    }
};

use super::ast::{ASTNode, Operator};


pub fn evaluate_ast<State: ReactiveState>(
    node: &ASTNode,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    match node {
        ASTNode::Number(num) => {
            Ok(Box::new(*num))
        },
        ASTNode::StringLiteral(string) => {
            Ok(Box::new(string.clone()))
        },
        ASTNode::Boolean(boolean) => {
            Ok(Box::new(*boolean))
        },
        ASTNode::Identifier(name) => 
            evaluate_identifier(name, state, for_loop_contexts),
        ASTNode::FunctionCall(name, args) =>
            evaluate_component_function(name, args.clone(), state, component_functions, for_loop_contexts),
        ASTNode::BinaryOperation { operator, left, right } =>
            evaluate_binary_operation(operator, left, right, state, component_functions, for_loop_contexts),
        ASTNode::Comparison { operator, left, right } =>
            evaluate_comparison(operator, left, right, state, component_functions, for_loop_contexts),
        ASTNode::LogicalOperation { operator, left, right } =>
            evaluate_logical_operation(operator, left, right, state, component_functions, for_loop_contexts),
    }
}

fn evaluate_identifier<State: ReactiveState>(
    name: &str,
    state: &State,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    let result = access_field(state, name, &Some(for_loop_contexts.clone()));
    if result.is_err() {
        println!("{:?}", result.err());
    }
    match access_field(state, name, &Some(for_loop_contexts.clone())) {
        Ok(val) => {
            if let Some(num) = val.as_any().downcast_ref::<f64>() {
                return Ok(Box::new(*num));
            }
            if let Some(int) = val.as_any().downcast_ref::<i64>() {
                return Ok(Box::new(*int));
            }
            if let Some(txt) = val.as_any().downcast_ref::<String>() {
                return Ok(Box::new(txt.clone()));
            }
            if let Some(boolean) = val.as_any().downcast_ref::<bool>() {
                return Ok(Box::new(*boolean));
            }
            Ok(val.as_any())
        }
        Err(_) => Err(ParsingError::ASTEvaluationError(format!("Field {} not found", name))),
    }
}

fn evaluate_component_function<State: ReactiveState>(
    name: &str,
    args: Vec<ASTNode>,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    let arg_values: Result<Vec<Box<dyn Any>>, ParsingError> = args.iter()
        .map(|arg| evaluate_ast(arg, state, component_functions, for_loop_contexts))
        .collect();

    match component_functions.dynamic_params_functions.get(name) {
        Some(func) => Ok(func(state, arg_values?)),
        None => Err(ParsingError::ASTEvaluationError(format!("Function {} not found in component functions", name))),
    }
}

fn evaluate_binary_operation<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    let left_val = evaluate_ast(left, state, component_functions, for_loop_contexts)?;
    let right_val = evaluate_ast(right, state, component_functions, for_loop_contexts)?;

    let left_float = left_val.downcast_ref::<f64>()
        .ok_or_else(|| ParsingError::ASTEvaluationError("Expected a floating point number on the left side of operation".to_string()))?;
    
    let right_float = *right_val.downcast_ref::<f64>()
        .ok_or_else(|| ParsingError::ASTEvaluationError("Expected a floating point number on the right side of operation".to_string()))?;

    let result = match operator {
        Operator::Add => left_float + right_float,
        Operator::Subtract => left_float - right_float,
        Operator::Multiply => left_float * right_float,
        Operator::Divide => left_float / right_float,
        _ => return Err(ParsingError::ASTEvaluationError("Unsupported operation for binary operation".to_string())),
    };

    Ok(Box::new(result)) 
}

fn evaluate_comparison<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    let left_val = evaluate_ast(left, state, component_functions, for_loop_contexts)?;
    let right_val = evaluate_ast(right, state, component_functions, for_loop_contexts)?;

    // Attempt to downcast and compare for: f64, i64, ParsingError, bool
    let result = left_val.downcast_ref::<f64>().and_then(|left_float| {
        right_val.downcast_ref::<f64>().and_then(|right_float| {
            compare_values(left_float, right_float, operator).ok()
        })
    }).or_else(|| {
        left_val.downcast_ref::<i64>().and_then(|left_int| {
            right_val.downcast_ref::<i64>().and_then(|right_int| {
                compare_values(left_int, right_int, operator).ok()
            })
        })
    }).or_else(|| {
        left_val.downcast_ref::<String>().and_then(|left_str| {
            right_val.downcast_ref::<String>().and_then(|right_str| {
                compare_values(left_str, right_str, operator).ok()
            })
        })
    }).or_else(|| {
        left_val.downcast_ref::<bool>().and_then(|left_bool| {
            right_val.downcast_ref::<bool>().and_then(|right_bool| {
                compare_values(left_bool, right_bool, operator).ok()
            })
        })
    });

    match result {
        Some(outcome) => Ok(Box::new(outcome)),
        None => Err(ParsingError::ASTEvaluationError("Type mismatch or unsupported comparison type".to_string()))
    }
}

fn compare_values<T: PartialOrd + PartialEq>(
    left: &T,
    right: &T,
    operator: &Operator,
) -> Result<bool, ParsingError> {
    match operator {
        Operator::Equal => Ok(left == right),
        Operator::NotEqual => Ok(left != right),
        Operator::Less => Ok(left < right),
        Operator::Greater => Ok(left > right),
        Operator::LessEqual => Ok(left <= right),
        Operator::GreaterEqual => Ok(left >= right),
        _ => Err(ParsingError::ASTEvaluationError("Unsupported operator for comparison".to_string())),
    }
}

fn evaluate_logical_operation<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    for_loop_contexts: &Vec<ForLoopContext>,
) -> Result<Box<dyn Any>, ParsingError> {
    let left_val = evaluate_ast(left, state, component_functions, for_loop_contexts)?;
    let right_val = evaluate_ast(right, state, component_functions, for_loop_contexts)?;

    let left_bool = *left_val.downcast_ref::<bool>()
        .ok_or_else(|| ParsingError::ASTEvaluationError("Type mismatch".to_string()))?;
    let right_bool = *right_val.downcast_ref::<bool>()
        .ok_or_else(|| ParsingError::ASTEvaluationError("Type mismatch".to_string()))?;

    let result = match operator {
        Operator::And => left_bool && right_bool,
        Operator::Or => left_bool || right_bool,
        _ => return Err(ParsingError::ASTEvaluationError("Unsupported operation for logical operation".to_string())),
    };

    Ok(Box::new(result)) 
}
