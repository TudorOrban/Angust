use std::{fs, path::PathBuf};


pub fn create_styles_directory(project_root_path: &PathBuf, styles_folder_path: &String) {
    let styles_dir_path = project_root_path.join(styles_folder_path);

    match fs::create_dir_all(&styles_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create styles directory: {}", e);
        }
    }

    create_styles_file(&styles_dir_path);
}

fn create_styles_file(styles_dir_path: &PathBuf) {
    let styles_css_path = styles_dir_path.join("styles.css");

    let styles_css_contents = r#"
body {
    font-family: Arial, sans-serif;
    background-color: rgb(0, 0, 0);
}
    "#;

    fs::write(&styles_css_path, styles_css_contents)
        .expect("Failed to write styles.css file");
}
