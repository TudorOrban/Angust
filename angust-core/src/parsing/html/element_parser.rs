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

use super::html_parser;

pub fn dispatch_element_processing(elem_data: &kuchiki::ElementData, node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    match elem_data.name.local.as_ref() {
        "div" => Some(process_div_element(elem_data, node, parent_styles, angust_config, stylesheet)),
        "button" => Some(process_button_element(elem_data, node, parent_styles, angust_config, stylesheet)),
        "img" => process_image_element(elem_data, node, parent_styles, angust_config, stylesheet),
        component_name => process_custom_component(component_name, elem_data, node, parent_styles, angust_config, stylesheet),
    }
}

fn process_div_element(elem_data: &kuchiki::ElementData, node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Box<dyn Element> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, stylesheet);
    container.set_styles(styles);

    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements(&child, Some(&styles), angust_config, stylesheet))
        .for_each(|child_element| container.add_child(child_element));

    Box::new(container)
}

fn process_button_element(elem_data: &kuchiki::ElementData, node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Box<dyn Element> {
    let attributes = elem_data.attributes.borrow();
    // let on_click = attributes.get("on_click").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, stylesheet);

    let mut button = Button::new(None, None, Some(styles));

    let mut child_container = Container::new();
    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements(&child, Some(&styles), angust_config, stylesheet))
        .for_each(|child_element| {
            child_container.add_child(child_element);
            child_container.set_styles(styles.clone());
        });
        
    button.add_child(Box::new(child_container));

    Box::new(button)
}

fn process_image_element(elem_data: &kuchiki::ElementData, _: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    let attributes = elem_data.attributes.borrow();
    let src = attributes.get("src").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, stylesheet);

    let image = Image::new(
        angust_config.pathing_config.assets_dir_path.clone() + "/img", src.to_string(), Some(styles)
    );
    Some(Box::new(image))
}

fn process_custom_component(component_name: &str, elem_data: &kuchiki::ElementData, node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    let skippable_components = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_components.contains(&component_name) {
        return html_parser::general_traversal(node, parent_styles, angust_config, stylesheet)
    }
    
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, stylesheet);

    println!("Processing custom component: {}", component_name);

    if let Some(mut component_box) = create_component(component_name) {
        println!("Component found: {}", component_name);
        component_box.set_styles(styles);
        Some(component_box)
    } else {
        println!("Component not found: {}", component_name);
        // // Continue processing children
        // // To be reported as an error in the future
        return html_parser::general_traversal(node, Some(&styles), angust_config, stylesheet)
    }
}