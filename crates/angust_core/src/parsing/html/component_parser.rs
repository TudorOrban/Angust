use std::collections::HashMap;

use crate::{
    parsing::{css::css_parser, directive::input::input_evaluator}, 
    rendering::{
        elements::{
            component::{component_factory_registry::create_component, state::reactivity::ReactiveState},
            element::Element, 
            styles::Styles
        }, 
        router::router_component::RouterComponent
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

    if component_name == "router-component" {
        return process_router_component(elem_data, parent_styles, context);
    }

    let component_optional = create_component(component_name);
    if component_optional.is_none() {
        return Err(ParsingError::ComponentNotFound(component_name.to_string()));
    }
    let mut component = component_optional.unwrap();
    
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    component.set_styles(styles);
    
    // Compute inputs using parent state and functions *before* initializing the component (i.e. parsing its template)
    let input_values = input_evaluator::compute_inputs_from_parent_component(&component, context)?;
    
    component.initialize(input_values);

    Ok(component)
}

fn process_router_component<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let attributes = elem_data.attributes.borrow();
    let mut component = Box::new(RouterComponent::new());
    
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    component.set_styles(styles);
    
    component.initialize(HashMap::new());

    Ok(component)
}