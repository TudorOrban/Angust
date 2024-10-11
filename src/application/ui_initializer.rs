
use crate::{parsing::html::html_parser::{map_dom_to_elements, parse_html_content}, rendering::elements::element::Element};

use super::{angust_configuration::AngustConfiguration, resource_loader::{html_loader, stylesheet_loader}};


pub fn initialize_ui(angust_config: &AngustConfiguration) -> Box<dyn Element> {
    let html_content = html_loader::load_index_html(
        angust_config.html_dir_relative_path.clone()
    ).unwrap_or_else(|| {
        panic!("Failed to load index.html")
    });

    let dom = parse_html_content(html_content.as_str());
    
    let stylesheets = stylesheet_loader::load_stylesheet(
        &angust_config.styles_dir_relative_path, &String::from("styles.css")
    ).unwrap_or_else(|| {
        panic!("Failed to load stylesheet")
    });
    println!("{}", stylesheets);

    map_dom_to_elements(&dom, None, angust_config)
        .expect("Failed to map DOM to elements")
}