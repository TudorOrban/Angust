use std::collections::HashMap;

use kuchiki::{
    parse_html,
    NodeData,
    NodeRef,
    traits::TendrilSink,
};

use crate::application::angust_configuration::AngustConfiguration;
use crate::parsing::{
    css::css_parser::merge_styles,
    css::stylesheet_parser::Stylesheet,
    directive::for_parser::ForLoopContext,
    directive::placeholder_parser,
    expression::ast::ASTNode,
};
use crate::rendering::elements::component::functions::component_functions::ComponentFunctions;
use crate::rendering::elements::{
    component::state::reactivity::ReactiveState,
    container::Container,
    element::Element,
    styles::Styles,
    text::Text,
};

use super::element_parser;
use super::error::ParsingError;


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
) -> Result<Box<dyn Element>, ParsingError> {
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
) -> Result<Box<dyn Element>, ParsingError> {
    let mut container = Container::new();
    for child in node.children() {
        let child_element = map_dom_to_elements::<State>(&child, parent_styles, context)?;
        container.add_child(child_element);
    }

    Ok(Box::new(container))
}


fn process_text_element<State : ReactiveState>(
    text: &str,
    parent_styles: Option<&Styles>,
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return Ok(Box::new(Container::new()));
    }

    // Apply state placeholders
    let final_text = match context.component_state {
        Some(state) => placeholder_parser::parse_state_placeholder(trimmed_text, state, context),
        None => Ok(trimmed_text.to_string()),
    }?;

    let mut text_element = Text::new(final_text);

    if let Some(styles) = parent_styles {
        let mut element_styles = Styles::default();
        merge_styles(styles, &mut element_styles);
        text_element.set_styles(element_styles);
    }
    
    Ok(Box::new(text_element))
}

pub fn general_traversal<State : ReactiveState>(
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let mut root_element: Option<Box<dyn Element>> = None;

    for child in node.children() {
        
        let child_element = map_dom_to_elements::<State>(&child, parent_styles, context)?;
        if root_element.is_none() {
            root_element = Some(child_element);
        } else {
            root_element.as_mut().unwrap().add_child(child_element);
        }
    }

    Ok(root_element.unwrap_or_else(|| Box::new(Container::new())))
}

pub struct ParsingContext<'a, State : ReactiveState> {
    pub angust_config: Option<AngustConfiguration>,
    pub stylesheet: Option<Stylesheet>,
    pub component_state: Option<&'a State>,
    pub component_functions: Option<&'a ComponentFunctions<State>>,
    pub template_asts: Option<TemplateASTs<'a>>,
    pub for_loop_contexts: Option<Vec<ForLoopContext>>,
}

impl<'a, State : ReactiveState> Default for ParsingContext<'a, State> {
    fn default() -> Self {
        ParsingContext {
            angust_config: None,
            stylesheet: None,
            component_state: None,
            component_functions: None,
            template_asts: None,
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
        input_expressions_asts: Option<&'a mut HashMap<String, ASTNode>>,
    ) -> Self {
        let template_asts = TemplateASTs::new(template_expressions_asts, template_event_handler_asts, input_expressions_asts);
        ParsingContext {
            angust_config,
            stylesheet,
            component_state,
            component_functions,
            template_asts: Some(template_asts),
            for_loop_contexts: None
        }
    }

    pub fn add_template_expression_ast(&mut self, ast: ASTNode) {
        if self.template_asts.is_none() {
            self.template_asts = Some(TemplateASTs::default());
        }
        let template_asts = self.template_asts.as_mut().unwrap();

        if let Some(template_expressions_asts) = &mut template_asts.template_expressions_asts {
            template_expressions_asts.push(ast);
        }
    }

    pub fn add_template_event_handler_ast(&mut self, event_name: String, ast: ASTNode) {
        if self.template_asts.is_none() {
            self.template_asts = Some(TemplateASTs::default());
        }
        let template_asts = self.template_asts.as_mut().unwrap();

        if let Some(template_event_handler_asts) = &mut template_asts.template_event_handler_asts {
            template_event_handler_asts.insert(event_name, ast);
        }
    }

    pub fn add_input_expression_ast(&mut self, input_name: String, ast: ASTNode) {
        if self.template_asts.is_none() {
            self.template_asts = Some(TemplateASTs::default());
        }
        let template_asts = self.template_asts.as_mut().unwrap();

        if let Some(input_expressions_asts) = &mut template_asts.input_expressions_asts {
            input_expressions_asts.insert(input_name, ast);
        }
    }

    pub fn add_for_loop_context(&mut self, context: ForLoopContext) {
        if let Some(for_loop_contexts) = &mut self.for_loop_contexts {
            for_loop_contexts.push(context);
        } else {
            self.for_loop_contexts = Some(vec![context]);
        }
    }

    pub fn increment_loop_index(&mut self, context_id: &str) {
        if let Some(for_loop_contexts) = &mut self.for_loop_contexts {
            let context = for_loop_contexts.iter_mut().find(|context| context.context_id == context_id);
            if let Some(context) = context {
                context.current_index += 1;
            }
        }
    }

    pub fn remove_loop_context(&mut self, context_id: &str) {
        if let Some(for_loop_contexts) = &mut self.for_loop_contexts {
            for_loop_contexts.retain(|context| context.context_id != context_id);
        }
    }
}

pub struct TemplateASTs<'a> {
    pub template_expressions_asts: Option<&'a mut Vec<ASTNode>>,
    pub template_event_handler_asts: Option<&'a mut HashMap<String, ASTNode>>,
    pub input_expressions_asts: Option<&'a mut HashMap<String, ASTNode>>,
}

impl<'a> Default for TemplateASTs<'a> {
    fn default() -> Self {
        TemplateASTs {
            template_expressions_asts: None,
            template_event_handler_asts: None,
            input_expressions_asts: None,
        }
    }
}

impl<'a> TemplateASTs<'a> {
    pub fn new(
        template_expressions_asts: Option<&'a mut Vec<ASTNode>>,
        template_event_handler_asts: Option<&'a mut HashMap<String, ASTNode>>,
        input_expressions_asts: Option<&'a mut HashMap<String, ASTNode>>,
    ) -> Self {
        TemplateASTs {
            template_expressions_asts,
            template_event_handler_asts,
            input_expressions_asts,
        }
    }
}