
use crate::{parsing::html::html_parser::{map_dom_to_elements, parse_html_content}, rendering::elements::element::Element};

use super::{angust_configuration::AngustConfiguration, resource_loader::html_loader};


pub fn initialize_ui(angust_config: &AngustConfiguration) -> Box<dyn Element> {
    let html_content = html_loader::load_index_html(
        angust_config.html_dir_relative_path.clone()
    ).unwrap_or_else(|| {
        panic!("Failed to load index.html")
    });

    let dom = parse_html_content(html_content.as_str());

    map_dom_to_elements(&dom, None)
        .expect("Failed to map DOM to elements")
}