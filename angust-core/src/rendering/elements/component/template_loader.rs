use std::path::PathBuf;

use crate::{
    application::resource_loader::path_navigator::identify_project_root_path, 
    parsing::html::html_parser::{self, ParsingContext}, 
    rendering::elements::{container::Container, element::Element}
};

use super::component::{Component, ComponentState};


pub fn load_component_template<State: ComponentState>(component: &mut Component<State>) {
    // Load template
    let project_root = PathBuf::from(identify_project_root_path());
    let template_path = project_root.join(component.template_relative_path.clone());

    let template_content = std::fs::read_to_string(template_path)
        .expect("Failed to read template file");

    // Parse template
    let dom = html_parser::parse_html_content(&template_content);

    let mut container = Box::new(Container::new());
    let parsing_context: ParsingContext<State> = html_parser::ParsingContext::new(
        None, 
        None, 
        Some(&component.state)
    );
    
    if let Some(element) = html_parser::map_dom_to_elements::<State>(&dom, None, &parsing_context) {
        container.add_child(element);
    }
    component.content = container;
}