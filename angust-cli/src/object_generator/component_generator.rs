use std::{env, fs, path::PathBuf};

use crate::shared::utils;


/*
 * Expects the current directory to be the project root
 * And the path to be of the form src/app/SomeComponent
 */
pub fn generate_component(path: &str) {
    let (
        component_dir_path, 
        component_rs_path, 
        path_to_html_from_root, 
        component_name, 
        kebab_case_component_name
    ) = process_path(path);

    create_component_directory(&component_dir_path);
    create_component_rs_file(
        &component_rs_path, &path_to_html_from_root, &component_name.to_string(), &kebab_case_component_name
    );
    create_component_template(&component_dir_path, &kebab_case_component_name);
}

fn process_path(path: &str) -> (PathBuf, PathBuf, PathBuf, String, String) {
    let current_dir_path = env::current_dir().expect("Failed to get current directory");
    let provided_path = PathBuf::from(path);

    let component_name = provided_path.file_name().unwrap().to_str().unwrap();
    let kebab_case_component_name = utils::string_pascal_to_kebab_case(component_name);

    let provided_path_dir = provided_path.parent().unwrap();
    let path_from_root = PathBuf::from("src").join(PathBuf::from("app")).join(provided_path_dir);
    let full_provided_path_dir = PathBuf::from(current_dir_path).join(path_from_root.clone());
    let component_dir_path = full_provided_path_dir.join(kebab_case_component_name.clone());
    
    let rs_file_name = format!("{}.component.rs", kebab_case_component_name);
    let component_rs_path = component_dir_path.join(rs_file_name);

    let html_file_name = format!("{}.component.html", kebab_case_component_name);
    let path_to_html_from_root = path_from_root.join(html_file_name);

    (component_dir_path, component_rs_path, path_to_html_from_root, component_name.to_string(), kebab_case_component_name)
}

fn create_component_directory(component_dir_path: &PathBuf) {
    match fs::create_dir_all(&component_dir_path) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to create component directory: {}", e);
        }
    }
}

fn create_component_rs_file(
    component_rs_path: &PathBuf, 
    path_to_html_from_root: &PathBuf,
    component_name: &String, 
    kebab_case_component_name: &String,
) {
    let path_to_html_from_root = path_to_html_from_root
        .to_str().unwrap()
        .replace("\\", "/");

    let component_component_contents = format!(r#"
use angust::rendering::elements::component::component::Component;

pub struct {component_name}Component {{
    component: Component<{component_name}ComponentState>,    
}}

pub struct {component_name}ComponentState {{
    content: String,
}}

impl {component_name}Component {{
    pub fn new() -> Self {{
        let state = {component_name}ComponentState {{ content: String::from("Hello, {component_name} Component!") }};

        let mut component = Component::new(
            "{kebab_case_component_name}".to_string(),
            "{path_to_html_from_root}".to_string(),
            state,
        );

        Self {{ component }}
    }}
}}
    "#);

    fs::write(&component_rs_path, component_component_contents)
        .expect("Failed to write component_component.rs file");
}

fn create_component_template(
    component_dir_path: &PathBuf,
    kebab_case_component_name: &String,

) {
    let component_template_path = component_dir_path.join(format!("{}.component.html", kebab_case_component_name));

    let component_template_contents = format!(r#"
<div style="background-color: rgb(255, 0, 0)">

    <div>{kebab_case_component_name} works!</div>
    <span>{{ content }}</span>

    <button @onclick="toggle">Toggle Content</button>
</div>
    "#);

    fs::write(&component_template_path, component_template_contents)
        .expect("Failed to write component.component.html file");
}
