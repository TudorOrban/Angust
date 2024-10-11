use std::{fs, path::PathBuf};

use crate::application::resource_loader::path_navigator;


pub fn load_stylesheet(stylesheet_directory_relative_path: String, stylesheet_file_relative_path: String) -> Option<String> {
    let path = 
        path_navigator::get_stylesheet_directory_path(stylesheet_directory_relative_path) + "/" +
        stylesheet_file_relative_path.as_str();

    fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content))
}