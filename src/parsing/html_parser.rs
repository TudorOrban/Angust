use kuchiki::parse_html;
use kuchiki::Attributes;
use kuchiki::NodeData;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::styles::Styles;

pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

pub fn map_dom_to_elements(dom: &NodeRef) -> Option<Box<dyn Element>> {
    let mut root_element: Option<Box<dyn Element>> = None;

    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => {
            // Continue to traverse children of document or doctype nodes
            for child in dom.children() {
                if let Some(element) = map_dom_to_elements(&child) {
                    return Some(element);
                }
            }
        },
        NodeData::Element(ref elem_data) if elem_data.name.local.as_ref() == "div" => {
            let mut container = Container::new();
            let attributes = elem_data.attributes.borrow();
            let styles = parse_styles(&attributes);
            container.set_styles(styles);

            // Recursively process child nodes and add them as children of this container
            for child in dom.children() {
                if let Some(child_element) = map_dom_to_elements(&child) {
                    container.add_child(child_element);
                }
            }
            println!("Container: {:?}, {:?}, {:?}", container.get_id(), container.get_position(), container.get_size());
            root_element = Some(Box::new(container));
        },
        NodeData::Text(ref text) => {
            let borrowed_text = text.borrow();
            let trimmed_text = borrowed_text.trim();
            if !trimmed_text.is_empty() {
                println!("Text: {}", trimmed_text);
            }
        },
        _ => {
            // Continue to traverse children of other node types
            for child in dom.children() {
                if let Some(element) = map_dom_to_elements(&child) {
                    if root_element.is_none() {
                        root_element = Some(element);
                    } else {
                        root_element.as_mut().unwrap().add_child(element);
                    }
                }
            }
        }
    }

    root_element
}


fn parse_styles(attributes: &Attributes) -> Styles {
    let mut styles = Styles::default();

    if let Some(class_name) = attributes.get("class") {
        styles = apply_class_styles(class_name);
    }

    if let Some(style_attr) = attributes.get("style") {
        styles = parse_inline_styles(style_attr);
    }

    styles
}

fn apply_class_styles(class_name: &str) -> Styles {
    Styles::default()
}

fn parse_inline_styles(style_str: &str) -> Styles {
    Styles::default()
}
