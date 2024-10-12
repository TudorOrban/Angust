use std::{fs, path::PathBuf};

use regex::Regex;

use crate::application::resource_loader::path_navigator;


pub fn load_stylesheet(styles_dir_relative_path: &String, stylesheet_relative_path: &String) -> Option<String> {
    let styles_relative_path = styles_dir_relative_path.clone() + "/" + stylesheet_relative_path;
    let path = path_navigator::get_styles_path(styles_relative_path);

    let initial_string = fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content));

    let import_regex = Regex::new(r#"@import\s+url\("([^"]+)"\);"#).unwrap();

    // Detect import statements and load the imported stylesheets
    let mut styles: String = String::new();

    if let Some(content) = initial_string {
        let lines = content.lines();

        for line in lines {
            if let Some(caps) = import_regex.captures(line) {
                let imported_path = caps.get(1).map_or("", |m| m.as_str()); // Capture the path inside the url()

                let imported_content = load_stylesheet(styles_dir_relative_path, &imported_path.to_string())
                    .unwrap_or_else(|| {
                        panic!("Failed to load imported stylesheet: {}", imported_path);
                    });

                styles.push_str(&imported_content);
                continue;
            }
            styles.push_str(line);
            styles.push('\n');
        }
    };

    Some(styles)
}