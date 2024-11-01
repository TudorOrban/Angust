use std::{any::Any, collections::HashMap, hash::Hash};

use crate::{
    parsing::{css::css_parser, directive::input_parser, expression::{ast::ASTNode, ast_evaluator}}, 
    rendering::elements::{
        component::{
            component_factory_registry::create_component, 
            functions::component_functions::ComponentFunctions, 
            state::reactivity::ReactiveState
        },
        element::{Element, ElementType}, 
        styles::Styles
    }
};

use super::{error::ParsingError, html_parser::{self, ParsingContext}};


pub fn process_custom_component<State : ReactiveState>(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &kuchiki::NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let skippable_elements = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_elements.contains(&component_name) {
        return html_parser::general_traversal::<State>(node, parent_styles, context)
    }

    let attributes = elem_data.attributes.borrow();
    input_parser::parse_input_expressions(&attributes, context)?;

    let component_optional = create_component(component_name);
    if component_optional.is_none() {
        return Err(ParsingError::ComponentNotFound(component_name.to_string()));
    }
    let mut component_box = component_optional.unwrap();
    
    let input_values = compute_inputs_from_parent_component(&component_box, context)?;

    component_box.initialize(input_values);

    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    component_box.set_styles(styles);

    Ok(component_box)
}

fn compute_inputs_from_parent_component<State: ReactiveState>(
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
        if *component_name != component_box.get_name() {
            continue;
        }

        let input_value = ast_evaluator::evaluate_ast(input_ast, state, &component_functions)?;

        input_values.insert(input_name.clone(), input_value);
    }

    Ok(input_values)
}

pub fn trigger_input_setters<State : ReactiveState>(
    element: &mut Box<dyn Element>,
    state: &State,
    component_functions: &ComponentFunctions<State>,
    context: &ParsingContext<State>,
) -> Result<(), ParsingError> {
    let mut empty_children: Vec<Box<dyn Element>> = vec![];

    let mut default_input_asts: HashMap<String, ASTNode> = HashMap::new();
    let reference = &mut default_input_asts;
    let input_asts = context.template_asts
        .as_ref()
        .and_then(|asts| asts.input_expressions_asts.as_ref())
        .unwrap_or(&reference);

    for child in element.get_children_mut().unwrap_or(&mut empty_children) {
        println!("Processing child with ID: {:?} and element type: {:?}", child.get_id(), child.get_element_type()); 
        if child.get_element_type() != ElementType::CustomComponent {
            trigger_input_setters(child, state, component_functions, context)?;
            continue;
        }

        let component_interface = child.get_component_interface();
        if component_interface.is_none() {
            println!("No component interface found");
            continue;
        }
        let comp_interface = component_interface.unwrap();
        
        println!("Input ASTs: {:?}", input_asts);
        for (input_name, input_ast) in input_asts.iter() {
            let input_value = ast_evaluator::evaluate_ast(input_ast, state, &component_functions)?;
            comp_interface.update_input(input_name, vec![input_value]);
        }
    }

    Ok(())
}