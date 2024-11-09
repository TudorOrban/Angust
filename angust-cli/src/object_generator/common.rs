use std::{
    env, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}
};

use crate::shared::utils;


/*
 * Processes path into several names needed for object generation
 * Expects the current directory to be the project root
 * And the path to be of the form src/app/SomeComponent (pascal case for the component name)
 */
pub fn process_path(path: &str, object_type: ObjectType) -> (PathBuf, PathBuf, PathBuf, PathBuf, String, String, String) {
    let provided_path = PathBuf::from(path);
    let current_dir_path = env::current_dir().expect("Failed to get current directory");

    let pascal_object_name = provided_path.file_name().unwrap().to_str().unwrap().to_owned() + object_type.get_name();
    let kebab_case_object_name = utils::string_pascal_to_kebab_case(&pascal_object_name);
    let snake_case_object_name = utils::string_pascal_to_snake_case(&pascal_object_name);
    
    let provided_path_dir = provided_path.parent().unwrap();
    let path_from_root: &std::path::Path = provided_path_dir; // Expand this in the future
    let full_provided_path_dir = PathBuf::from(current_dir_path.clone()).join(path_from_root);
    let object_dir_path = full_provided_path_dir.join(snake_case_object_name.clone());
    
    let rs_file_name = format!("{}.rs", snake_case_object_name);
    let object_rs_path = object_dir_path.join(rs_file_name);

    let html_file_name = format!("{}/{}.html", snake_case_object_name, snake_case_object_name);
    let path_to_html_from_root = path_from_root.join(html_file_name);

    (current_dir_path, object_dir_path, object_rs_path, path_to_html_from_root, pascal_object_name.to_string(), kebab_case_object_name, snake_case_object_name)
}

/*
 * Ensures a generated object is in a declared module,
 * by creating/updating all the folders and corresponding mod.rs files for a given object directory
 * object can be: Component, Service
 */
pub fn create_object_module(object_dir_path: &PathBuf, current_dir_path: &PathBuf) {
    let base_path = current_dir_path.join("src").join("app");  // Starting point inside src/app
    let relative_path = object_dir_path.strip_prefix(&base_path).unwrap();

    let mut current_path = base_path.clone();
    let mut previous_mod_path = Some(base_path.join("mod.rs"));  // mod.rs in app directory

    for object in relative_path.iter() {
        current_path.push(object);

        if !current_path.exists() {
            fs::create_dir_all(&current_path).expect("Failed to create directory");
        }

        if let Some(ref mod_path) = previous_mod_path {
            let module_name = object.to_str().unwrap();
            update_mod_file(mod_path, module_name);
        }

        previous_mod_path = Some(current_path.join("mod.rs"));
    }

    // Ensure the last object's mod.rs is updated to include the object file
    if let Some(ref final_mod_path) = previous_mod_path {
        let file_stem = object_dir_path.file_stem().unwrap().to_str().unwrap();
        update_mod_file(final_mod_path, file_stem);
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Component,
    Service,
}

impl ObjectType {
    pub fn get_name(&self) -> &'static str {
        match self {
            ObjectType::Component => "Component",
            ObjectType::Service => "Service",
        }
    }
}