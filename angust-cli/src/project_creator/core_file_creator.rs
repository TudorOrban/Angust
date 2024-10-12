use std::{fs, path::PathBuf};



pub fn create_core_files(project_root_path: &PathBuf, main_rs_path: &String, index_html_path: &String) {
    update_main_rs_file(project_root_path, main_rs_path); // Already created by cargo init
    create_index_html_file(project_root_path, index_html_path);
}

fn update_main_rs_file(project_root_path: &PathBuf, main_rs_path: &String) {
    let main_rs_path = project_root_path.join(main_rs_path);

    let main_rs_contents = r#"
extern crate angust;

use angust::application::application::Application;


pub struct AppGlobalState {
    pub message: String,
}

fn main() {
    let initial_state = AppGlobalState {
        message: "Hello, Angust user!".to_string(),
    };

    let mut app = Application::new(initial_state, String::from("New Angust App"));
    
    app.run();
}
    "#;

    fs::write(&main_rs_path, main_rs_contents)
        .expect("Failed to write main.rs file");
}


fn create_index_html_file(project_root_path: &PathBuf, index_html_path: &String) {
    let index_html_path = project_root_path.join(index_html_path);

    let index_html_contents = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Angust App</title>
</head>
<body>
    <div>
        <app-component></app-component>
    </div>
</body>
</html>
"#;

    fs::write(&index_html_path, index_html_contents)
        .expect("Failed to write index.html file");
}