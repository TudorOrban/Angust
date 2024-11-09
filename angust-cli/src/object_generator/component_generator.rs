use std::{env, fs::{self, File}, io::{Read, Write}, path::PathBuf};

use crate::shared::utils;

use super::common::create_object_module;


/*
 * Expects the current directory to be the project root
 * And the path to be of the form src/app/SomeComponent (pascal case for the component name)
 */
pub fn generate_component(path: &str) {
    let (
        current_dir_path,
        component_dir_path, 
        component_rs_path, 
        path_to_html_from_root, 
        pascal_case_component_name, 
        kebab_case_component_name,
        snake_case_component_name
    ) = process_path(path);

    create_object_module(&component_dir_path, &current_dir_path);
    create_component_rs_file(
        &component_rs_path, &path_to_html_from_root, &pascal_case_component_name.to_string(), &kebab_case_component_name
    );
    create_component_template(&component_dir_path, &snake_case_component_name, &kebab_case_component_name);
    update_component_registration_module(&component_rs_path, &current_dir_path, &pascal_case_component_name);
}

fn process_path(path: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf, String, String, String) {
    let provided_path = PathBuf::from(path);
    let current_dir_path = env::current_dir().expect("Failed to get current directory");

    let pascal_component_name = provided_path.file_name().unwrap().to_str().unwrap().to_owned() + "Component";
    let kebab_case_component_name = utils::string_pascal_to_kebab_case(&pascal_component_name);
    let snake_case_component_name = utils::string_pascal_to_snake_case(&pascal_component_name);
    
    let provided_path_dir = provided_path.parent().unwrap();
    let path_from_root: &std::path::Path = provided_path_dir; // Expand this in the future
    let full_provided_path_dir = PathBuf::from(current_dir_path.clone()).join(path_from_root);
    let component_dir_path = full_provided_path_dir.join(snake_case_component_name.clone());
    
    let rs_file_name = format!("{}.rs", snake_case_component_name);
    let component_rs_path = component_dir_path.join(rs_file_name);

    let html_file_name = format!("{}.html", snake_case_component_name);
    let path_to_html_from_root = path_from_root.join(html_file_name);

    (current_dir_path, component_dir_path, component_rs_path, path_to_html_from_root, pascal_component_name.to_string(), kebab_case_component_name, snake_case_component_name)
}

fn create_component_rs_file(
    component_rs_path: &PathBuf, 
    path_to_html_from_root: &PathBuf,
    pascal_case_component_name: &String, 
    kebab_case_component_name: &String,
) {
    let path_to_html_from_root = path_to_html_from_root // TODO: This is missing an extra kebab_case_component_name from directory
        .to_str().unwrap()
        .replace("\\", "/");

    let component_component_contents = format!(r#"
use std::collections::HashMap;

use angust::rendering::elements::component::{{
    component::Component, 
    component_factory_registry::ComponentFactory, 
}};
use angust_macros::component_state;


#[component_state]
struct {pascal_case_component_name}State {{
    content: String,
}}

impl {pascal_case_component_name}State {{

}}

pub struct {pascal_case_component_name} {{

}}

impl {pascal_case_component_name} {{
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {{
        registry.insert("{kebab_case_component_name}".to_string(), Box::new(move || {{
            let state_factory = || {pascal_case_component_name}State::new(
                "{kebab_case_component_name} works!".to_string(),
            );

            let component = Component::new(
                "{kebab_case_component_name}".to_string(),
                "{path_to_html_from_root}".to_string(),
                state_factory() 
            );

            Box::new(component)
        }}));
    }}
}}
    "#);

    fs::write(&component_rs_path, component_component_contents)
        .expect("Failed to write component_component.rs file");
}

fn create_component_template(
    component_dir_path: &PathBuf,
    snake_case_component_name: &String,
    kebab_case_component_name: &String,
) {
    let component_template_path = component_dir_path.join(format!("{}.html", snake_case_component_name));

    let component_template_contents = format!(r#"
<div>
    {kebab_case_component_name} works!
</div>
    "#);

    fs::write(&component_template_path, component_template_contents)
        .expect("Failed to write component.component.html file");
}



fn update_component_registration_module(
    component_rs_path: &PathBuf, 
    current_dir_path: &PathBuf,
    pascal_case_component_name: &str
) {
    let relative_path = component_rs_path.strip_prefix(current_dir_path.join("src")).unwrap();
    let import_path = relative_path.to_str().unwrap()
                        .trim_end_matches(".rs")
                        .replace("\\", "/");  // Normalize path
    let module_path = import_path.replace("/", "::");

    let import_statement = format!("use crate::{}::{};", module_path, pascal_case_component_name);
    let register_call = format!("    {}::register(&mut registry);", pascal_case_component_name);

    let component_registration_file_path = current_dir_path.join("src").join("component_registration.rs");

    let mut contents = String::new();
    if component_registration_file_path.exists() {
        File::open(&component_registration_file_path).unwrap().read_to_string(&mut contents).unwrap();
    }

    // Insert the import statement if it does not exist
    if !contents.contains(&import_statement) {
        let last_use_crate_index = contents.rfind("use crate").map(|idx| contents[idx..].find('\n').unwrap() + idx + 1).unwrap_or(0);
        contents.insert_str(last_use_crate_index, &format!("{}\n", import_statement));
    }

    // Ensure all registration calls are correctly placed before the initialization call
    let init_call_index = contents.find("initialize_component_registry(registry);").unwrap();
    if !contents[..init_call_index].contains(&register_call) {
        let last_register_index = contents[..init_call_index].rfind(';').unwrap() + 1;
        contents.insert_str(last_register_index, &format!("\n{}", register_call));
    }

    // Properly indent the initialize component registry line
    let init_line_start = contents.rfind("initialize_component_registry(registry);").unwrap();
    contents.replace_range(init_line_start..init_line_start, "");

    // Write the updated contents back to the file
    File::create(&component_registration_file_path).unwrap().write_all(contents.as_bytes()).unwrap();
}
