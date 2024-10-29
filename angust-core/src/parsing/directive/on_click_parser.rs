use crate::{
    parsing::{
        expression::ast::{self, ASTNode},
        html::{error::ParsingError, html_parser::ParsingContext}
    }, 
    rendering::elements::component::state::reactivity::ReactiveState
};

use super::id_generator::IDGenerator;



// On click attribute @onclick="event_handler_name()"
pub fn parse_on_click_expression<State: ReactiveState>(
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<(String, ASTNode), ParsingError> {
    let on_click_attribute = match parse_on_click_attribute::<State>(attributes, context) {
        Some(expr) => expr,
        None => return Err(ParsingError::InvalidDirectiveSyntax("No on click attribute found".to_string())),
    };

    let ast = ast::parse_string_to_ast(on_click_attribute.clone())
        .map_err(|e| ParsingError::ASTParsingError(format!("Error parsing on click expression: {:?}", e)))?;

    // Get root function name
    let mut root_function_name = match ast.clone() {
        ASTNode::FunctionCall(function_name, _) => function_name,
        _ => return Err(ParsingError::InvalidDirective("Invalid on click expression".to_string())),
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

