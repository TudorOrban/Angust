use std::collections::HashMap;

use kuchiki::{NodeData, NodeRef};

use crate::parsing::{
    expression::ast::{self, ASTNode}, 
    html::error::ParsingError
};

use super::input_parser;


/*
 * Function that prescans the DOM for input expressions for depth 1 component children.
 */
pub fn scan_inputs(
    dom: &NodeRef,
) -> Result<HashMap<(String, String), ASTNode>, ParsingError> {
    let mut inputs_map: HashMap<(String, String), ASTNode> = HashMap::new(); // (component_id, property_name) -> ASTNode

    scan_dom_for_inputs(dom, &mut inputs_map)?;

    Ok(inputs_map)
}

fn scan_dom_for_inputs(
    dom: &NodeRef,
    inputs_map: &mut HashMap<(String, String), ASTNode>,
) -> Result<(), ParsingError> {
    match dom.data() {
        // NodeData::Document(_) | NodeData::Doctype(_) => 
        //     scan_node_children_for_inputs(dom, inputs_map),
        NodeData::Element(ref elem_data) => {
            match elem_data.name.local.as_ref() {
                "div" | "button" | "img" => scan_node_children_for_inputs(dom, inputs_map),
                component_name => scan_component_inputs(component_name, elem_data, dom, inputs_map),
            }
        }
        _ => scan_node_children_for_inputs(dom, inputs_map),
    }
}

fn scan_node_children_for_inputs(
    node: &kuchiki::NodeRef,
    inputs_map: &mut HashMap<(String, String), ASTNode>,
) -> Result<(), ParsingError> {
    for child in node.children() {
        scan_dom_for_inputs(&child, inputs_map)?;
    }

    Ok(())
}

fn scan_component_inputs(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &kuchiki::NodeRef,
    inputs_map: &mut HashMap<(String, String), ASTNode>,
) -> Result<(), ParsingError> {
    let skippable_elements = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_elements.contains(&component_name) {
        return scan_node_children_for_inputs(node, inputs_map);
    }

    let attributes = elem_data.attributes.borrow();
    let inputs = input_parser::parse_input_attributes(&attributes);

    for (property_name, bound_value) in inputs.iter() {
        let ast = ast::parse_string_to_ast(bound_value.to_string())
            .map_err(|e| ParsingError::ASTParsingError(format!("{:?}", e)))?;

        println!("Input AST: {:?}", ast);

        inputs_map.insert((component_name.to_string(), property_name.clone()), ast);
    }

    Ok(())
}