use std::{env, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};

use crate::shared::utils;


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

    create_component_module(&component_dir_path, &current_dir_path);
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

fn create_component_module(component_dir_path: &PathBuf, current_dir_path: &PathBuf) {
    let base_path = current_dir_path.join("src").join("app");  // Starting point inside src/app
    let relative_path = component_dir_path.strip_prefix(&base_path).unwrap();

    let mut current_path = base_path.clone();
    let mut previous_mod_path = Some(base_path.join("mod.rs"));

    for component in relative_path.iter() {
        current_path.push(component);

        if !current_path.exists() {
            fs::create_dir_all(&current_path).expect("Failed to create directory");
        }

        if let Some(ref mod_path) = previous_mod_path {
            let module_name = component.to_str().unwrap();
            update_mod_file(mod_path, module_name);
        }

        previous_mod_path = Some(current_path.join("mod.rs"));
    }
}

fn update_mod_file(mod_file_path: &Path, module_name: &str) {
    if !mod_file_path.exists() {
        let mut mod_file = File::create(mod_file_path).unwrap();
        writeln!(mod_file, "pub mod {};", module_name).expect("Failed to write to mod.rs");
    } else {
        let mut contents = String::new();
        File::open(mod_file_path).unwrap().read_to_string(&mut contents).unwrap();
        if !contents.contains(&format!("pub mod {};", module_name)) {
            let mut mod_file = File::options().append(true).open(mod_file_path).unwrap();
            writeln!(mod_file, "pub mod {};", module_name).expect("Failed to write to mod.rs");
        }
    }
}

fn create_component_rs_file(
    component_rs_path: &PathBuf, 
    path_to_html_from_root: &PathBuf,
    pascal_case_component_name: &String, 
    kebab_case_component_name: &String,
) {
    let path_to_html_from_root = path_to_html_from_root
        .to_str().unwrap()
        .replace("\\", "/");

    let component_component_contents = format!(r#"
use angust::rendering::elements::component::component::Component;

pub struct {pascal_case_component_name} {{
    component: Component<{pascal_case_component_name}State>,    
}}

pub struct {pascal_case_component_name}State {{
    content: String,
}}

impl {pascal_case_component_name} {{
    pub fn register() {{
        let state_factory = || {pascal_case_component_name}State::new();

        register_component("{kebab_case_component_name}".to_string(), Box::new(move || {{
            Component::new(
                "{kebab_case_component_name}".to_string(),
                "{path_to_html_from_root}".to_string(),
                state_factory() 
            )
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
<div style="background-color: rgb(255, 0, 0)">

    <div>{kebab_case_component_name} works!</div>
    <span>{{ content }}</span>

    <button @onclick="toggle">Toggle Content</button>
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
    let register_call = format!("    {}::register();", pascal_case_component_name);

    let component_registration_file_path = current_dir_path.join("src").join("component_registration.rs");

    let mut contents = String::new();
    if component_registration_file_path.exists() {
        File::open(&component_registration_file_path).unwrap().read_to_string(&mut contents).unwrap();
    }

    let mut new_contents = String::new();

    // Find the last `use crate` occurrence and place the new import after it
    let last_use_crate_index = contents.rfind("use crate").map(|idx| contents[idx..].find('\n').unwrap() + idx + 1).unwrap_or(0);

    new_contents.push_str(&contents[..last_use_crate_index]);
    if !contents.contains(&import_statement) {
        new_contents.push_str(&format!("{}\n", import_statement));
    }
    new_contents.push_str(&contents[last_use_crate_index..]);

    // Add the register call within the function block
    let function_start_index = new_contents.find("pub fn register_components()").unwrap();
    let function_body_start = new_contents[function_start_index..].find('{').unwrap() + function_start_index + 1;
    let function_body_end = new_contents[function_start_index..].rfind('}').unwrap() + function_start_index;

    let before_register_block = new_contents[..function_body_start].to_string();
    let register_block = new_contents[function_body_start..function_body_end].to_string();
    let after_register_block = new_contents[function_body_end..].to_string();

    if !register_block.contains(&register_call) {
        let updated_register_block = format!("{}\n{}\n", register_block, register_call);
        new_contents = format!("{}{}{}", before_register_block, updated_register_block, after_register_block);
    }

    // Write the updated contents back to the file
    File::create(&component_registration_file_path).unwrap().write_all(new_contents.as_bytes()).unwrap();
}