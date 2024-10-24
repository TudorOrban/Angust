use std::any::Any;

use crate::rendering::elements::component::{component_state::{get_nested_field, ReactiveState}, functions::component_functions::ComponentFunctions};

use super::ast::{ASTNode, Operator};


pub fn evaluate_ast<State: ReactiveState>(
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
        ASTNode::Boolean(boolean) => {
            Ok(Box::new(*boolean))
        },
        ASTNode::Identifier(name) => 
            evaluate_identifier_new(name, state),
        ASTNode::FunctionCall(_, _) => {
            // evaluate_component_function_new(name, args.clone(), state, functions)
            Err("Function calls are not supported yet".to_string())
        },
        ASTNode::BinaryOperation { operator, left, right } =>
            evaluate_binary_operation_new(operator, left, right, state, functions),
        ASTNode::Comparison { operator, left, right } =>
            evaluate_comparison_new(operator, left, right, state, functions),
        ASTNode::LogicalOperation { operator, left, right } =>
            evaluate_logical_operation_new(operator, left, right, state, functions),
    }
}

fn evaluate_identifier_new<State: ReactiveState>(
    name: &str,
    state: &State,
) -> Result<Box<dyn Any>, String> {
    match get_nested_field(state, &[name]) {
        Some(val) => {
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
            Ok(Box::new(val.as_any()))
        }
        None => Err(format!("Field {} not found", name)),
    }
}

// fn evaluate_component_function_new<State: ReactiveState>(
//     name: &str,
//     args: Vec<ASTNode>,
//     state: &State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<Box<dyn Any>, String> {
//     let arg_values: Result<Vec<Box<dyn Any>>, String> = args.iter()
//         .map(|arg| evaluate_ast(arg, state, functions))
//         .collect();

//     match functions.dynamic_params_functions.get(name) {
//         Some(func) => Ok(func(state, arg_values?)),
//         None => Err(format!("Function {} not found in component functions", name)),
//     }
// }

fn evaluate_binary_operation_new<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
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

    Ok(Box::new(result)) 
}

fn evaluate_comparison_new<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    // Attempt to downcast and compare for: f64, i64, String, bool
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
        None => Err("Type mismatch or unsupported comparison type".to_string())
    }
}

fn compare_values<T: PartialOrd + PartialEq>(
    left: &T,
    right: &T,
    operator: &Operator,
) -> Result<bool, String> {
    match operator {
        Operator::Equal => Ok(left == right),
        Operator::NotEqual => Ok(left != right),
        Operator::Less => Ok(left < right),
        Operator::Greater => Ok(left > right),
        Operator::LessEqual => Ok(left <= right),
        Operator::GreaterEqual => Ok(left >= right),
        _ => Err("Unsupported operator for comparison".to_string()),
    }
}

fn evaluate_logical_operation_new<State: ReactiveState>(
    operator: &Operator,
    left: &ASTNode,
    right: &ASTNode,
    state: &State,
    functions: &ComponentFunctions<State>,
) -> Result<Box<dyn Any>, String> {
    let left_val = evaluate_ast(left, state, functions)?;
    let right_val = evaluate_ast(right, state, functions)?;

    let left_bool = *left_val.downcast_ref::<bool>().ok_or("Type mismatch")?;
    let right_bool = *right_val.downcast_ref::<bool>().ok_or("Type mismatch")?;

    let result = match operator {
        Operator::And => left_bool && right_bool,
        Operator::Or => left_bool || right_bool,
        _ => return Err("Unsupported operation for logical operation".to_string()),
    };

    Ok(Box::new(result)) 
}

// pub fn evaluate_ast<'a, State: ReactiveState>(
//     node: &ASTNode,
//     state: &'a State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<&'a dyn Any, String> {
//     match node {
//         ASTNode::Number(num) => {
//             let leaked_value = Box::leak(Box::new(*num) as Box<dyn Any>);
//             Ok(leaked_value as &'a dyn Any)
//         },
//         ASTNode::StringLiteral(string) => {
//             let leaked_value = Box::leak(Box::new(string.clone()) as Box<dyn Any>);
//             Ok(leaked_value as &'a dyn Any)
//         },
//         ASTNode::Boolean(boolean) => {
//             let leaked_value = Box::leak(Box::new(*boolean) as Box<dyn Any>);
//             Ok(leaked_value as &'a dyn Any)
//         },
//         ASTNode::Identifier(name) => {
//             match get_nested_field(state, &[name]) {
//                 Some(val) => {
//                     Ok(val.as_any())
//                 }
//                 None => Err(format!("Field {} not found", name)),
//             }
//         },
//         ASTNode::FunctionCall(name, args) => {
//             evaluate_component_function(name, args.clone(), state, functions)
//         },
//         ASTNode::BinaryOperation { operator, left, right } =>
//             evaluate_binary_operation(operator, left, right, state, functions),
//         ASTNode::Comparison { operator, left, right } =>
//             evaluate_comparison(operator, left, right, state, functions),
//         ASTNode::LogicalOperation { operator, left, right } =>
//             evaluate_logical_operation(operator, left, right, state, functions),
//     }
// }


// fn evaluate_component_function<'a, State: ReactiveState>(
//     name: &str,
//     args: Vec<ASTNode>,
//     state: &'a State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<&'a dyn Any, String> {
//     let arg_values: Result<Vec<&dyn Any>, String> = args.iter()
//         .map(|arg| evaluate_ast(arg, state, functions))
//         .collect();

//     match functions.dynamic_params_functions.get(name) {
//         Some(func) => Ok(func(state, arg_values?)),
//         None => Err(format!("Function {} not found in component functions", name)),
//     }
// }

// fn evaluate_binary_operation<'a, State: ReactiveState>(
//     operator: &Operator,
//     left: &ASTNode,
//     right: &ASTNode,
//     state: &'a State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<&'a dyn Any, String> {
//     let left_val = evaluate_ast(left, state, functions)?;
//     let right_val = evaluate_ast(right, state, functions)?;

//     let left_float = *left_val.downcast_ref::<f64>().ok_or("Type mismatch")?;
//     let right_float = *right_val.downcast_ref::<f64>().ok_or("Type mismatch")?;

//     let result = match operator {
//         Operator::Add => left_float + right_float,
//         Operator::Subtract => left_float - right_float,
//         Operator::Multiply => left_float * right_float,
//         Operator::Divide => left_float / right_float,
//         _ => return Err("Unsupported operation for binary operation".to_string()),
//     };

//     Ok(Box::leak(Box::new(result)) as &'a dyn Any) 
// }

// fn evaluate_comparison<'a, State: ReactiveState>(
//     operator: &Operator,
//     left: &ASTNode,
//     right: &ASTNode,
//     state: &'a State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<&'a dyn Any, String> {
//     let left_val = evaluate_ast(left, state, functions)?;
//     let right_val = evaluate_ast(right, state, functions)?;

//     if let Some(result) = try_numeric_comparison(operator, left_val, right_val)? {
//         return Ok(Box::leak(Box::new(result)) as &dyn Any);  
//     }

//     if let Some(result) = try_string_comparison(operator, left_val, right_val)? {
//         return Ok(Box::leak(Box::new(result)) as &dyn Any);
//     }

//     Err("Type mismatch or unsupported comparison type".to_string())
// }

// fn try_numeric_comparison<'a>(
//     operator: &Operator,
//     left_val: &'a dyn Any,
//     right_val: &'a dyn Any,
// ) -> Result<Option<bool>, String> {
//     if let (Some(left_float), Some(right_float)) = (
//         left_val.downcast_ref::<f64>(),
//         right_val.downcast_ref::<f64>(),
//     ) {
//         let result = match operator {
//             Operator::Equal => left_float == right_float,
//             Operator::NotEqual => left_float != right_float,
//             Operator::Less => left_float < right_float,
//             Operator::Greater => left_float > right_float,
//             Operator::LessEqual => left_float <= right_float,
//             Operator::GreaterEqual => left_float >= right_float,
//             _ => return Err("Unsupported operator for numeric comparison".to_string()),
//         };

//         return Ok(Some(result));
//     }
//     Ok(None)
// }

// fn try_string_comparison<'a>(
//     operator: &Operator,
//     left_val: &'a dyn Any,
//     right_val: &'a dyn Any,
// ) -> Result<Option<bool>, String> {
//     if let (Some(left_str), Some(right_str)) = (
//         left_val.downcast_ref::<String>(),
//         right_val.downcast_ref::<String>(),
//     ) {
//         let result = match operator {
//             Operator::Equal => left_str == right_str,
//             Operator::NotEqual => left_str != right_str,
//             _ => return Err("String comparison only supports Equal and NotEqual".to_string()),
//         };

//         return Ok(Some(result));
//     }
//     Ok(None)
// }


// fn evaluate_logical_operation<'a, State: ReactiveState>(
//     operator: &Operator,
//     left: &ASTNode,
//     right: &ASTNode,
//     state: &'a State,
//     functions: &ComponentFunctions<State>,
// ) -> Result<&'a dyn Any, String> {
//     let left_val = evaluate_ast(left, state, functions)?;
//     let right_val = evaluate_ast(right, state, functions)?;

//     let left_bool = *left_val.downcast_ref::<bool>().ok_or("Type mismatch")?;
//     let right_bool = *right_val.downcast_ref::<bool>().ok_or("Type mismatch")?;

//     let result = match operator {
//         Operator::And => left_bool && right_bool,
//         Operator::Or => left_bool || right_bool,
//         _ => return Err("Unsupported operation for logical operation".to_string()),
//     };

//     Ok(Box::leak(Box::new(result)) as &'a dyn Any) 
// }
