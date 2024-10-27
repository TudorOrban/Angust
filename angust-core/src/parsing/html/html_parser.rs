use std::collections::HashMap;

use kuchiki::parse_html;
use kuchiki::NodeData;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use crate::application::angust_configuration::AngustConfiguration;
use crate::parsing::css::css_parser::merge_styles;
use crate::parsing::css::stylesheet_parser::Stylesheet;
use crate::parsing::expression::ast::ASTNode;
use crate::rendering::elements::component::component_state::ReactiveState;
use crate::rendering::elements::component::functions::component_functions::ComponentFunctions;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::styles::Styles;
use crate::rendering::elements::text::Text;

use super::directive_parser;
use super::directive_parser::ForLoopContext;
use super::element_parser;


pub fn parse_html_content(html: &str) -> NodeRef {
    parse_html().one(html)
}

/*
 * Function that maps the parsed DOM into a tree of Angust elements.
 */
pub fn map_dom_to_elements<State : ReactiveState>(
    dom: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => 
            process_document_nodes::<State>(dom, parent_styles, context),
        NodeData::Element(ref elem_data) => 
            element_parser::dispatch_element_processing::<State>(elem_data, dom, parent_styles, context),
        NodeData::Text(ref text) => 
            process_text_element::<State>(&text.borrow(), parent_styles, context),
        _ => general_traversal::<State>(dom, parent_styles, context),
    }
}

fn process_document_nodes<State : ReactiveState>(
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    node.children()
        .filter_map(|child| map_dom_to_elements::<State>(&child, parent_styles, context))
        .next()
}


fn process_text_element<State : ReactiveState>(
    text: &str,
    parent_styles: Option<&Styles>,
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return None
    }

    // Apply state placeholders
    let final_text = match context.component_state {
        Some(state) => directive_parser::parse_state_placeholder(trimmed_text, state, context)
            .unwrap_or_else(|er| {
                println!("Error parsing state placeholders in text element: {}", er);
                trimmed_text.to_string()
            }),
        None => trimmed_text.to_string(),
    };

    let mut text_element = Text::new(final_text);
    if let Some(styles) = parent_styles {
        let mut element_styles = Styles::default();
        merge_styles(styles, &mut element_styles);
        text_element.set_styles(element_styles);
    }
    Some(Box::new(text_element))
}

pub fn general_traversal<State : ReactiveState>(
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
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

pub struct ParsingContext<'a, State : ReactiveState> {
    pub angust_config: Option<AngustConfiguration>,
    pub stylesheet: Option<Stylesheet>,
    pub component_state: Option<&'a State>,
    pub component_functions: Option<&'a ComponentFunctions<State>>,
    pub template_expressions_asts: Option<&'a mut Vec<ASTNode>>,
    pub template_event_handler_asts: Option<&'a mut HashMap<String, ASTNode>>,
    pub for_loop_contexts: Option<Vec<ForLoopContext>>,
}

impl<'a, State : ReactiveState> Default for ParsingContext<'a, State> {
    fn default() -> Self {
        ParsingContext {
            angust_config: None,
            stylesheet: None,
            component_state: None,
            component_functions: None,
            template_expressions_asts: None,
            template_event_handler_asts: None,
            for_loop_contexts: None,
        }
    }
}

impl<'a, State : ReactiveState> ParsingContext<'a, State> {
    pub fn new(
        angust_config: Option<AngustConfiguration>,
        stylesheet: Option<Stylesheet>,
        component_state: Option<&'a State>,
        component_functions: Option<&'a ComponentFunctions<State>>,
        template_expressions_asts: Option<&'a mut Vec<ASTNode>>,
        template_event_handler_asts: Option<&'a mut HashMap<String, ASTNode>>,
    ) -> Self {
        ParsingContext {
            angust_config,
            stylesheet,
            component_state,
            component_functions,
            template_expressions_asts,
            template_event_handler_asts,
            for_loop_contexts: None
        }
    }

    pub fn add_template_expression_ast(&mut self, ast: ASTNode) {
        if let Some(template_expressions_asts) = &mut self.template_expressions_asts {
            template_expressions_asts.push(ast);
        }
    }

    pub fn add_template_event_handler_ast(&mut self, event_name: String, ast: ASTNode) {
        if let Some(template_event_handler_asts) = &mut self.template_event_handler_asts {
            template_event_handler_asts.insert(event_name, ast);
        }
    }

    pub fn add_for_loop_context(&mut self, context: ForLoopContext) {
        if let Some(for_loop_contexts) = &mut self.for_loop_contexts {
            for_loop_contexts.push(context);
        } else {
            self.for_loop_contexts = Some(vec![context]);
        }
    }
}