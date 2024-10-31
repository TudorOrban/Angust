use crate::{
    parsing::{css::css_parser, directive::input_parser, expression::ast_evaluator}, 
    rendering::elements::{
        component::{component_factory_registry::create_component, functions::component_functions::ComponentFunctions, state::{reactivity::ReactiveState, reflectivity::{NoState, ReflectiveState}}}, 
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

    let component_optional = create_component(component_name);
    if component_optional.is_none() {
        return Err(ParsingError::ComponentNotFound(component_name.to_string()));
    }
    let mut component_box = component_optional.unwrap();

    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    component_box.set_styles(styles);

    input_parser::parse_input_expressions(&attributes, context)?;

    trigger_input_setters(&mut component_box, context)?;

    Ok(component_box)
}

fn trigger_input_setters<State : ReactiveState>(
    component: &mut Box<dyn Element>,
    context: &ParsingContext<State>,
) -> Result<(), ParsingError> {
    let component_state = match component.get_state() {
        Some(ref state) => *state,
        None => return Ok(()) // Root context, do nothing
    };
    let component_functions: ComponentFunctions<NoState> = ComponentFunctions::default();
    // let component_functions = match context.component_functions {
    //     Some(ref functions) => *functions,
    //     None => return Ok(())
    // };

    let mut empty_children: Vec<Box<dyn Element>> = vec![];

    println!("Triggering input setters for component: {:?}", component.get_id());
    println!("Processing component: {:?}", component.get_element_type());

    for child in component.get_children_mut().unwrap_or(&mut empty_children) {
        println!("Processing child with ID: {:?} and element type: {:?}", child.get_id(), child.get_element_type()); 
        if child.get_element_type() != ElementType::CustomComponent {
            trigger_input_setters(child, context)?;
            continue;
        }

        let component_interface = child.get_component_interface();
        if component_interface.is_none() {
            println!("No component interface found");
            continue;
        }
        let comp_interface = component_interface.unwrap();
        
        let input_asts = comp_interface.get_input_asts();
        
        println!("Input ASTs: {:?}", input_asts);
        for (input_name, input_ast) in input_asts.iter() {
            let input_value = ast_evaluator::evaluate_ast(input_ast, component_state, &component_functions)?;
            comp_interface.update_input(input_name, vec![input_value]);
        }
        
    }

    Ok(())
}