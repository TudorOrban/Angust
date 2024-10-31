use std::path::PathBuf;

use crate::{
    application::resource_loader::path_navigator::identify_project_root_path, 
    parsing::html::{component_parser::trigger_input_setters, html_parser::{self, ParsingContext}}, 
    rendering::elements::{container::Container, element::Element}
};

use super::{component::Component, state::reactivity::ReactiveState};


// Entry point of Component Template parsing
pub fn load_component_template<'a, State: ReactiveState>(component: &'a mut Component<State>) {
    // Load template
    let project_root = PathBuf::from(identify_project_root_path());
    let template_path = project_root.join(component.template_relative_path.clone());

    let template_content = std::fs::read_to_string(template_path)
        .expect("Failed to read template file");
    
    // Parse template HTML content
    let dom = html_parser::parse_html_content(&template_content);

    // Map Kuchiki DOM to elements
    let mut container = Box::new(Container::new());
    let mut parsing_context: ParsingContext<'a, State> = html_parser::ParsingContext::new(
        None, None, 
        Some(&component.state), Some(&component.component_functions),
        Some(&mut component.template_expressions_asts), 
        Some(&mut component.template_event_handler_asts),
        Some(&mut component.input_expressions_asts),
    );

    let mut element = html_parser::map_dom_to_elements::<State>(&dom, None, &mut parsing_context)
        .unwrap_or_else(|e| panic!("Failed to map DOM to elements: {:?}", e));
    
    trigger_input_setters::<State>(&mut element, &component.state, &component.component_functions, &parsing_context);


    // Add elements to Angust DOM
    container.add_child(element);

    println!("Component template loaded: {:?}", component.template_relative_path);


    component.content = container;
}