use std::{env, fs, path::PathBuf};

use crate::parsing::html_parser::{map_dom_to_elements, parse_html_content};

use super::elements::element::Element;

pub fn fetch_ui_body() -> Box<dyn Element> {
    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        // Fallback: Use the directory where the executable is located
        env::current_exe()
            .expect("Failed to find executable path")
            .parent()
            .expect("Failed to resolve executable directory")
            .to_path_buf()
            .display()
            .to_string()
    });

    let mut path = PathBuf::from(project_root);
    path.push("resources/index.html"); // Append the relative path to index.html

    let html_content = fs::read_to_string(path)
        .expect("Failed to read HTML content");

    let dom = parse_html_content(html_content.as_str());

    map_dom_to_elements(&dom)
        .expect("Failed to map DOM to elements")
}