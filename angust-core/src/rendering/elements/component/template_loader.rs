use std::path::PathBuf;

use crate::{application::{angust_configuration::AngustConfiguration, resource_loader::path_navigator::identify_project_root_path}, parsing::{css::stylesheet_parser::Stylesheet, html::html_parser}, rendering::elements::{container::Container, element::Element}};

use super::component::Component;


pub fn load_template<State>(component: &mut Component<State>) {
    // Load template
    let project_root = PathBuf::from(identify_project_root_path());
    let template_path = project_root.join(component.template_relative_path.clone());

    let template_content = std::fs::read_to_string(template_path)
        .expect("Failed to read template file");

    // Parse template
    let dom = html_parser::parse_html_content(&template_content);

    let mut container = Box::new(Container::new());
    if let Some(element) = html_parser::map_dom_to_elements(&dom, None, &AngustConfiguration::default(), &Stylesheet::default()) {
        container.add_child(element);
    }
    component.content = container;
}