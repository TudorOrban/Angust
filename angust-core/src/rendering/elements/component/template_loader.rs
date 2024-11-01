use std::{any::Any, collections::HashMap, path::PathBuf};

use crate::{
    application::resource_loader::path_navigator::identify_project_root_path, 
    parsing::{
        directive::input_scanner, 
        expression::{ast::ASTNode, ast_evaluator}, 
        html::{error::ParsingError, html_parser::{self, ParsingContext}}
    }, 
    rendering::elements::{container::Container, element::{Element, ElementType}}
};

use super::{component::Component, functions::component_functions::ComponentFunctions, state::reactivity::ReactiveState};


// Entry point of Component Template parsing
pub fn load_component_template<'a, State: ReactiveState>(component: &'a mut Component<State>, inputs: HashMap<String, Box<dyn Any>>) {
    // Load template
    let project_root = PathBuf::from(identify_project_root_path());
    let template_path = project_root.join(component.template_relative_path.clone());

    let template_content = std::fs::read_to_string(template_path)
        .expect("Failed to read template file");
    
    // Parse template HTML content
    let dom = html_parser::parse_html_content(&template_content);

    // Scan for inputs of (depth 1) children components
    let scanned_inputs = input_scanner::scan_inputs(&dom)
        .unwrap_or_else(|e| panic!("Failed to scan inputs: {:?}", e));

    // Trigger setters for inputs from parent component
    for (input_name, input_value) in inputs.into_iter() {
        let setter_name = format!("set_{}", input_name);

        let setter = component.component_functions.input_setters.get(&setter_name);
        if setter.is_none() {
            continue;
        }
        let setter = setter.unwrap();

        let vec = vec![input_value];
        println!("Reached this");
        setter(&mut component.state, vec);
    }

    // Map Kuchiki DOM to elements
    let mut container = Box::new(Container::new());
    let mut parsing_context: ParsingContext<'a, State> = html_parser::ParsingContext::new(
        None, None, 
        Some(&component.state), Some(&component.component_functions),
        Some(&mut component.template_expressions_asts), 
        Some(&mut component.template_event_handler_asts),
        Some(&mut component.input_expressions_asts),
        Some(scanned_inputs),
    );

    let element = html_parser::map_dom_to_elements::<State>(&dom, None, &mut parsing_context)
        .unwrap_or_else(|e| panic!("Failed to map DOM to elements: {:?}", e));
    

    // // Trigger input setters
    // trigger_children_input_setters(&mut element, &component.state, &component.component_functions, &parsing_context, &scanned_inputs)
    //     .unwrap_or_else(|e| panic!("Failed to trigger input setters: {:?}", e));
    
    // Add elements to Angust DOM
    container.add_child(element);

    println!("Component template loaded: {:?}", component.template_relative_path);


    component.content = container;
}

// fn trigger_children_input_setters<State: ReactiveState>(
//     element: &mut Box<dyn Element>, 
//     state: &State, 
//     component_functions: &ComponentFunctions<State>, 
//     parsing_context: &ParsingContext<State>,
//     scanned_inputs: &HashMap<(String, String), ASTNode>,
// ) -> Result<(), ParsingError> {
//     let mut empty_children: Vec<Box<dyn Element>> = vec![];

//     for child in element.get_children_mut().unwrap_or(&mut empty_children) {
//         if child.get_element_type() != ElementType::CustomComponent {
//             trigger_children_input_setters(child, state, component_functions, parsing_context, scanned_inputs)?;
//             continue;
//         }

//         let child_id = child.get_id().clone();

//         let component_interface = child.get_component_interface();
//         if component_interface.is_none() {
//             println!("No component interface found");
//             continue;
//         }
//         let comp_interface = component_interface.unwrap();
        
//         // Filter and collect input ASTs that are meant for this child component
//         let input_asts: HashMap<String, ASTNode> = scanned_inputs.iter()
//             .filter_map(|((comp_id, input_name), ast)| {
//                 if comp_id == &child_id {
//                     Some((input_name.clone(), ast.clone()))
//                 } else {
//                     None
//                 }
//             })
//             .collect();

            
//         let attributes = elem_data.attributes.borrow();
//         let inputs = input_parser::parse_input_attributes(&attributes);

//         for (property_name, bound_value) in inputs.iter() {
//             let ast = ast::parse_string_to_ast(bound_value.to_string())
//                 .map_err(|e| ParsingError::ASTParsingError(format!("{:?}", e)))?;

//             println!("Input AST: {:?}", ast);

//             inputs_map.insert((component_name.to_string(), property_name.clone()), ast);
//         }


//         println!("Input ASTs: {:?}", input_asts);
//         for (input_name, input_ast) in input_asts.iter() {
//             let input_value = ast_evaluator::evaluate_ast(input_ast, state, &component_functions)?;
//             comp_interface.update_input(input_name, vec![input_value]);
//         }
//     }

//     Ok(())
// }