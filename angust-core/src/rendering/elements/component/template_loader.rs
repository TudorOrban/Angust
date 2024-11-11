use std::{any::Any, collections::HashMap, path::PathBuf};

use crate::{
    application::resource_loader::path_navigator::identify_project_root_path, 
    parsing::{
        directive::input::{input_scanner, input_setter}, 
        html::html_parser::{self, print_dom_structure, ParsingContext}
    }, 
    rendering::elements::{container::Container, element::Element}
};

use super::{component::Component, state::reactivity::ReactiveState};


// Entry point of Component Template parsing
pub fn load_component_template<'a, State: ReactiveState>(component: &'a mut Component<State>, inputs: HashMap<String, Box<dyn Any>>) {
    // Load template
    let project_root = PathBuf::from(identify_project_root_path());
    let template_path = project_root.join(component.template_relative_path.clone());

    println!("Loading template: {}", template_path.display());
    let template_content = std::fs::read_to_string(template_path.clone())
        .expect(format!("Failed to read template file: {}", template_path.display()).as_str());

    println!("Template content: {}", template_content);
    // Parse template HTML content
    let dom = html_parser::parse_html_content(&template_content);

    // print_dom_structure(&dom);

    // Scan for inputs of (depth 1) children components
    let scanned_inputs = input_scanner::scan_inputs(&dom)
        .unwrap_or_else(|e| panic!("Failed to scan inputs: {:?}", e));

    println!("Name: {}, Scanned inputs: {:?}", component.name, scanned_inputs);

    // Trigger setters for inputs from parent component *before* mapping DOM to elements
    input_setter::trigger_input_setters(component, inputs);

    // Map Kuchiki DOM to elements
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

    // Add elements to Angust DOM
    let mut container = Box::new(Container::new());
    container.add_child(element);

    component.content = container;
}
