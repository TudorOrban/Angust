use crate::{
    parsing::{
        expression::{ast, ast_evaluator}, 
        html::html_parser::ParsingContext
    }, 
    rendering::elements::component::component_state::ReactiveState
};


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