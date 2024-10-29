use crate::{
    parsing::{
        expression::{ast, ast_evaluator}, 
        html::{error::ParsingError, html_parser::ParsingContext}
    }, 
    rendering::elements::component::state::reactivity::ReactiveState
};


// If directive @if="expression"
pub fn parse_if_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<bool, ParsingError> {
    let if_expression = match parse_if_attribute::<State>(attributes) {
        Some(expr) => expr,
        None => return Ok(true), // No if directive found
    };

    let ast = ast::parse_string_to_ast(if_expression)
        .map_err(|e| ParsingError::ASTParsingError(format!("{:?}", e)))?;
    ParsingContext::add_template_expression_ast(context, ast.clone());

    let state = context.component_state.expect("Component state not found");
    let functions = context.component_functions.expect("Component functions not found");
    let evaluation_result = ast_evaluator::evaluate_ast::<State>(&ast, state, functions)
        .map_err(|e| ParsingError::ASTEvaluationError(e))?;

    let is_if_true = evaluation_result
        .downcast_ref::<bool>()
        .ok_or_else(|| ParsingError::ASTEvaluationError(String::from("If expression did not evaluate to a boolean")))?;

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