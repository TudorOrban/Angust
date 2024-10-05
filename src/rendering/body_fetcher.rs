use std::{env, fs, path::PathBuf};

use crate::{parsing::html_parser::{parse_html_content, traverse}, rendering::elements::{common_types::Position, container::Container}};

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
    println!("Path: {:?}", path);
    let html_content = fs::read_to_string(path)
        .expect("Failed to read HTML content");
    let document = parse_html_content(html_content.as_str());

    traverse(&document);

    Box::new(Container::new())
}