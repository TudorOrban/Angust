use kuchiki::parse_html;
use kuchiki::NodeData;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use crate::application::angust_configuration::AngustConfiguration;
use crate::parsing::css::css_parser;
use crate::parsing::css::css_parser::merge_styles;
use crate::parsing::css::stylesheet_parser::Stylesheet;
use crate::rendering::elements::button::Button;
use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::image::Image;
use crate::rendering::elements::styles::Styles;
use crate::rendering::elements::text::Text;


pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

/*
 * Map that transforms the DOM into a tree of Angust elements.
 */
pub fn map_dom_to_elements(dom: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => process_document_nodes(dom, parent_styles, angust_config, stylesheet),
        NodeData::Element(ref elem_data) if elem_data.name.local.as_ref() == "div" => {
            Some(process_div_element(elem_data, dom, parent_styles, angust_config, stylesheet))
        },
        NodeData::Element(ref elem_data) if elem_data.name.local.as_ref() == "button" => {
            Some(process_button_element(elem_data, dom, parent_styles, angust_config, stylesheet))
        },
        NodeData::Element(ref elem_data) if elem_data.name.local.as_ref() == "img" => {
            process_image_element(elem_data, dom, parent_styles, angust_config, stylesheet)
        },
        NodeData::Text(ref text) => {
            process_text_element(&text.borrow(), parent_styles, angust_config)
        },
        _ => general_traversal(dom, parent_styles, angust_config, stylesheet),
    }
}

fn process_document_nodes(node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    node.children()
        .filter_map(|child| map_dom_to_elements(&child, parent_styles, angust_config, stylesheet))
        .next()
}

fn process_div_element(elem_data: &kuchiki::ElementData, node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Box<dyn Element> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, stylesheet);
    container.set_styles(styles);

    node.children()
        .filter_map(|child| map_dom_to_elements(&child, Some(&styles), angust_config, stylesheet))
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
        .filter_map(|child| map_dom_to_elements(&child, Some(&styles), angust_config, stylesheet))
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
        angust_config.images_dir_relative_path.clone(), src.to_string(), Some(styles)
    );
    Some(Box::new(image))
}

fn process_text_element(text: &str, parent_styles: Option<&Styles>, _: &AngustConfiguration) -> Option<Box<dyn Element>> {
    let trimmed_text = text.trim();
    if !trimmed_text.is_empty() {
        let mut text_element = Text::new(trimmed_text.to_string());
        if let Some(styles) = parent_styles {
            let mut element_styles = Styles::default();
            merge_styles(styles, &mut element_styles);
            text_element.set_styles(element_styles);
        }
        Some(Box::new(text_element))
    } else {
        None
    }
}

fn general_traversal(node: &NodeRef, parent_styles: Option<&Styles>, angust_config: &AngustConfiguration, stylesheet: &Stylesheet) -> Option<Box<dyn Element>> {
    let mut root_element: Option<Box<dyn Element>> = None;

    for child in node.children() {
        if let Some(element) = map_dom_to_elements(&child, parent_styles, angust_config, stylesheet) {
            if root_element.is_none() {
                root_element = Some(element);
            } else {
                root_element.as_mut().unwrap().add_child(element);
            }
        }
    }

    root_element
}

