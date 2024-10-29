use crate::{
    parsing::{css::css_parser, directive::input_parser}, 
    rendering::elements::{
        component::{component_factory_registry::create_component, state::reactivity::ReactiveState}, 
        element::Element, 
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
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let component_optional = create_component(component_name);
    if component_optional.is_none() {
        return Err(ParsingError::ComponentNotFound(component_name.to_string()));
    }
    let mut component_box = component_optional.unwrap();

    component_box.set_styles(styles);

    let input_results = input_parser::parse_input_expressions(&attributes, context)?;

    for (property_name, value) in input_results.iter() {
        // let input_setter_opt = component_box.
    }

    Ok(component_box)
}