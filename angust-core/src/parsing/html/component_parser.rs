use crate::{
    parsing::{css::css_parser, directive::input_parser}, 
    rendering::elements::{
        component::{self, component_factory_registry::create_component, state::reactivity::ReactiveState}, 
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

    Ok(component_box)
}

fn trigger_input_setters<State : ReactiveState>(
    component: &mut Box<dyn Element>,
    attributes: &kuchiki::Attributes,
    context: &mut ParsingContext<State>,
) -> Result<(), ParsingError> {
    for child in component.get_children_mut().unwrap() {
        if child.get_element_type() == ElementType::CustomComponent {
            if let Some(comp_interface) = child.get_component_interface() {
                comp_interface.update_input(input_name, value);
            }
        }
    }

    Ok(())
}