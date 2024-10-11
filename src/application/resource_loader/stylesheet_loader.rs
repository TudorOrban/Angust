use std::{fs, path::PathBuf};

use crate::application::resource_loader::path_navigator;


pub fn load_stylesheet(styles_relative_path: String) -> Option<String> {
    let path = path_navigator::get_styles_path(styles_relative_path);

    let initial_string = fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content));

    // Detect import statements and load the imported stylesheets
    let mut styles = String::new();

    if let Some(content) = initial_string {
        let lines = content.lines();

        for line in lines {
            if !line.starts_with("@import") {
                styles.push_str(line);
                continue;
            }

            let imported_path = line.split_whitespace().last().unwrap();
            let imported_content = fs::read_to_string(PathBuf::from(imported_path))
                .unwrap_or_else(|_| {
                    panic!("Failed to load imported stylesheet")
                });

            styles.push_str(imported_content.as_str());
        }
    };

    Some(styles)
}