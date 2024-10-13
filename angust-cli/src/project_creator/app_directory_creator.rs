use std::{fs, path::PathBuf};


pub fn create_app_directory(project_root_path: &PathBuf, app_folder_path: &String) {
    let app_dir_path = project_root_path.join(app_folder_path);

    match fs::create_dir_all(&app_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create app directory: {}", e);
        }
    }

    create_app_mod_file(&app_dir_path);
    create_app_component(&app_dir_path);
    create_app_template(&app_dir_path);
}

fn create_app_mod_file(app_dir_path: &PathBuf) {
    let app_mod_path = app_dir_path.join("mod.rs");

    let app_mod_contents = r#"
pub mod app_component;
    "#;

    fs::write(&app_mod_path, app_mod_contents)
        .expect("Failed to write app mod.rs file");
}

fn create_app_component(app_dir_path: &PathBuf) {
    let app_component_path = app_dir_path.join("app_component.rs");

    let app_component_contents = r#"
use std::collections::HashMap;

use angust::rendering::elements::component::{component::Component, component_factory::ComponentFactory};


pub struct AppComponent {
    component: Component<AppComponentState>,    
}

#[derive(Clone)]
pub struct AppComponentState {
    content: String,
}

impl AppComponentState {
    fn new() -> Self {
        Self { content: String::from("Hello, App Component!") }
    }
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || AppComponentState::new();

        registry.insert("app-component".to_string(), Box::new(move || {
            Box::new(
                Component::new(
                    "app-component".to_string(),
                    "src/app/app_component.html".to_string(),
                    state_factory() 
                )
            )
        }));
    }
}
    "#;

    fs::write(&app_component_path, app_component_contents)
        .expect("Failed to write app_component.rs file");
}

fn create_app_template(app_dir_path: &PathBuf) {
    let app_template_path = app_dir_path.join("app_component.html");

    let app_template_contents = r#"
<div style="background-color: rgb(50, 50, 220)">

    <h1>{{ content }}</h1>

    <button @onclick="toggle">Toggle Content</button>
</div>
    "#;

    fs::write(&app_template_path, app_template_contents)
        .expect("Failed to write app_component.html file");
}
