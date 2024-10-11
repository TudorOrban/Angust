use std::{fs, path::PathBuf};

use super::path_navigator;

pub fn load_index_html(html_directory_relative_path: String) -> Option<String> {
    load_html(html_directory_relative_path, "index.html".to_string())
}

pub fn load_html(html_directory_relative_path: String, html_file_relative_path: String) -> Option<String> {
    let path = 
        path_navigator::get_html_directory_path(html_directory_relative_path) + "/" +
        html_file_relative_path.as_str();

    fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content))
}