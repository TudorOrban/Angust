use std::{any::Any, collections::HashMap};

use kuchiki::{NodeData, NodeRef};

use crate::{parsing::html::{error::ParsingError, html_parser::{general_traversal, process_document_nodes, ParsingContext}}, rendering::elements::{component::state::reactivity::ReactiveState, styles::Styles}};



pub fn scan_inputs<State: ReactiveState>(
    dom: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<(), ParsingError> {
    let mut inputs_map: HashMap<(String, String), Box<dyn Any>> = HashMap::new(); // (component_id, property_name) -> ASTNode

    match dom.data() {
        NodeData::Document(_) | NodeData::Doctype(_) => 
            process_document_nodes::<State>(dom, parent_styles, context),
        NodeData::Element(ref elem_data) => {
            match elem_data.name.local.as_ref() {
                "div" => general_traversal::<State>(dom, parent_styles, context),
                "button" => general_traversal::<State>(dom, parent_styles, context),
                "img" => general_traversal::<State>(dom, parent_styles, context),
                component_name => scan_component_inputs::<State>(component_name, elem_data, dom, parent_styles, context, &mut inputs_map),
            }
        }
        _ => general_traversal::<State>(dom, parent_styles, context),
    }

    Ok(())
}

fn scan_component_inputs<State: ReactiveState>(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &kuchiki::NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
    inputs_map: &mut HashMap<(String, String), Box<dyn Any>>,
) -> Result<(), ParsingError> {
    

    Ok(())
}