use kuchiki::NodeRef;

use crate::application::angust_configuration::AngustConfiguration;
use crate::parsing::css::css_parser;
use crate::parsing::css::stylesheet_parser::Stylesheet;
use crate::rendering::elements::button::Button;
use crate::rendering::elements::component::component_factory::create_component;
use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::image::Image;
use crate::rendering::elements::styles::Styles;

use super::html_parser::{self, ParsingContext};

pub fn dispatch_element_processing<State>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    match elem_data.name.local.as_ref() {
        "div" => Some(process_div_element(elem_data, node, parent_styles, context)),
        "button" => Some(process_button_element(elem_data, node, parent_styles, context)),
        "img" => process_image_element(elem_data, node, parent_styles, context),
        component_name => process_custom_component(component_name, elem_data, node, parent_styles, context),
    }
}

fn process_div_element<State>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Box<dyn Element> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    container.set_styles(styles);

    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements(&child, Some(&styles), context))
        .for_each(|child_element| container.add_child(child_element));

    Box::new(container)
}

fn process_button_element<State>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Box<dyn Element> {
    let attributes = elem_data.attributes.borrow();
    // let on_click = attributes.get("on_click").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let mut button = Button::new(None, None, Some(styles));

    let mut child_container = Container::new();
    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements(&child, Some(&styles), context))
        .for_each(|child_element| {
            child_container.add_child(child_element);
            child_container.set_styles(styles.clone());
        });
        
    button.add_child(Box::new(child_container));

    Box::new(button)
}

fn process_image_element<State>(
    elem_data: &kuchiki::ElementData, 
    _: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let attributes = elem_data.attributes.borrow();
    let src = attributes.get("src").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let relative_path = context.angust_config.clone().unwrap_or_default().pathing_config.assets_dir_path.to_string() + "/img";
    let image = Image::new(
        relative_path, src.to_string(), Some(styles)
    );
    Some(Box::new(image))
}

fn process_custom_component<State>(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let skippable_elements = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_elements.contains(&component_name) {
        return html_parser::general_traversal(node, parent_styles, context)
    }
    
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    if let Some(mut component_box) = create_component(component_name) {
        component_box.set_styles(styles);
        Some(component_box)
    } else {
        // Continue processing children (To be reported as an error in the future)
        println!("Component not found: {}", component_name);
        return html_parser::general_traversal(node, Some(&styles), context)
    }
}