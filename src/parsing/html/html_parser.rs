use kuchiki::parse_html;
use kuchiki::NodeData;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use crate::parsing::css::css_parser;
use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;


pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

/*
 * Map that transforms the DOM into a tree of Reast elements.
 */
pub fn map_dom_to_elements(dom: &NodeRef) -> Option<Box<dyn Element>> {
    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => process_document_nodes(dom),
        NodeData::Element(ref elem_data) if elem_data.name.local.as_ref() == "div" => {
            Some(process_div_element(elem_data, dom))
        },
        NodeData::Text(ref text) => {
            handle_text_node(&text.borrow());
            None
        },
        _ => general_traversal(dom),
    }
}

fn process_document_nodes(node: &NodeRef) -> Option<Box<dyn Element>> {
    node.children()
        .filter_map(|child| map_dom_to_elements(&child))
        .next()
}

fn process_div_element(elem_data: &kuchiki::ElementData, node: &NodeRef) -> Box<dyn Element> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes);
    container.set_styles(styles);

    node.children()
        .filter_map(|child| map_dom_to_elements(&child))
        .for_each(|child_element| container.add_child(child_element));

    Box::new(container)
}

fn handle_text_node(text: &str) {
    let trimmed_text = text.trim();
    if !trimmed_text.is_empty() {
        // println!("Text: {}", trimmed_text);
    }
}

fn general_traversal(node: &NodeRef) -> Option<Box<dyn Element>> {
    let mut root_element: Option<Box<dyn Element>> = None;

    for child in node.children() {
        if let Some(element) = map_dom_to_elements(&child) {
            if root_element.is_none() {
                root_element = Some(element);
            } else {
                root_element.as_mut().unwrap().add_child(element);
            }
        }
    }

    root_element
}