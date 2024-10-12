
use kuchiki::NodeRef;

use crate::parsing::html::html_parser::parse_html_content;

use super::{angust_configuration::AngustConfiguration, resource_loader::{html_loader, stylesheet_loader}};


pub fn load_resources(angust_config: &AngustConfiguration) -> (NodeRef, String) {
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


    (dom, stylesheets)
}