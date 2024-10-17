use kuchiki::parse_html;
use kuchiki::NodeData;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use crate::application::angust_configuration::AngustConfiguration;
use crate::parsing::css::css_parser::merge_styles;
use crate::parsing::css::stylesheet_parser::Stylesheet;
use crate::rendering::elements::component::component_state::ComponentState;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::styles::Styles;
use crate::rendering::elements::text::Text;

use super::directive_parser;
use super::element_parser;


pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

/*
 * Function that maps the parsed DOM into a tree of Angust elements.
 */
pub fn map_dom_to_elements<State : ComponentState>(
    dom: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => process_document_nodes::<State>(dom, parent_styles, context),
        NodeData::Element(ref elem_data) => {
            element_parser::dispatch_element_processing::<State>(elem_data, dom, parent_styles, context)
        },
        NodeData::Text(ref text) => {
            process_text_element::<State>(&text.borrow(), parent_styles, context)
        },
        _ => general_traversal::<State>(dom, parent_styles, context),
    }
}

fn process_document_nodes<State : ComponentState>(
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    node.children()
        .filter_map(|child| map_dom_to_elements::<State>(&child, parent_styles, context))
        .next()
}


fn process_text_element<State : ComponentState>(
    text: &str,
    parent_styles: Option<&Styles>,
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return None
    }

    // Apply state placeholders
    let final_text = match context.component_state {
        Some(state) => directive_parser::parse_state_placeholder(trimmed_text, state)
            .unwrap_or_else(|_| trimmed_text.to_string()),
        None => trimmed_text.to_string(),
    };
    println!("Final text: {}", final_text);

    let mut text_element = Text::new(final_text);
    if let Some(styles) = parent_styles {
        let mut element_styles = Styles::default();
        merge_styles(styles, &mut element_styles);
        text_element.set_styles(element_styles);
    }
    Some(Box::new(text_element))
}

pub fn general_traversal<State : ComponentState>(
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let mut root_element: Option<Box<dyn Element>> = None;

    for child in node.children() {
        if let Some(element) = map_dom_to_elements::<State>(&child, parent_styles, context) {
            if root_element.is_none() {
                root_element = Some(element);
            } else {
                root_element.as_mut().unwrap().add_child(element);
            }
        }
    }

    root_element
}

pub struct ParsingContext<'a, State : ComponentState> {
    pub angust_config: Option<AngustConfiguration>,
    pub stylesheet: Option<Stylesheet>,
    pub component_state: Option<&'a State>,
}

impl<'a, State : ComponentState> Default for ParsingContext<'a, State> {
    fn default() -> Self {
        ParsingContext {
            angust_config: None,
            stylesheet: None,
            component_state: None,
        }
    }
}

impl<'a, State : ComponentState> ParsingContext<'a, State> {
    pub fn new(
        angust_config: Option<AngustConfiguration>,
        stylesheet: Option<Stylesheet>,
        component_state: Option<&'a State>,
    ) -> Self {
        ParsingContext {
            angust_config,
            stylesheet,
            component_state,
        }
    }
}