use std::{env, fs, path::PathBuf};

use crate::shared::utils;


/*
 * Expects the current directory to be the project root
 * And the path to be of the form src/app/SomeComponent
 */
pub fn generate_component(path: &str) {
    println!("Component {} would be generated here.", path);

    // Expecting the current directory to be the project root
    // And the path to be under the src/app directory
    let current_dir_path = env::current_dir().expect("Failed to get current directory");
    let provided_path = PathBuf::from(path);

    let component_name = provided_path.file_name().unwrap().to_str().unwrap();
    let kebab_case_component_name = utils::string_pascal_to_kebab_case(component_name);

    let provided_path_dir = provided_path.parent().unwrap();
    let full_provided_path_dir = PathBuf::from(current_dir_path).join("src/app").join(provided_path_dir);
    let component_dir_path = full_provided_path_dir.join(kebab_case_component_name.clone());

    create_component_directory(&component_dir_path);
    create_component_rs_file(&component_dir_path);
    create_component_template(&component_dir_path);
}

pub fn create_component_directory(component_dir_path: &PathBuf) {
    match fs::create_dir_all(&component_dir_path) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to create component directory: {}", e);
        }
    }
}

fn create_component_rs_file(component_dir_path: &PathBuf, component_name: &String, kebab_case_component_name: &String) {

    let component_component_contents = format!(r#"
use angust::rendering::elements::component::component::Component;

pub struct ComponentComponent {{
    component: Component<ComponentComponentState>,    
}}

pub struct ComponentComponentState {{
    content: String,
}}

impl ComponentComponent {{
    pub fn new() -> Self {{
        let state = ComponentComponentState {{ content: String::from("Hello, Component Component!") }};

        let mut component = Component::new(
            "{kebab_case_component_name}".to_string(),
            "src/app/{component_dir_path}/{kebab_case_component_name}.component.html".to_string(),
            state,
        );

        Self {{ component }}
    }}
}}
    "#);

    fs::write(&component_component_path, component_component_contents)
        .expect("Failed to write component_component.rs file");
}

fn create_component_template(component_dir_path: &PathBuf) {
    let component_template_path = component_dir_path.join("component.component.html");

    let component_template_contents = r#"
<div style="background-color: rgb(255, 0, 0)">

    <h1>{{ content }}</h1>

    <button @onclick="toggle">Toggle Content</button>
</div>
    "#;

    fs::write(&component_template_path, component_template_contents)
        .expect("Failed to write component.component.html file");
}
