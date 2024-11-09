
use std::{fs, path::PathBuf};

use super::common::{create_object_module, process_path, ObjectType};




pub fn generate_service(path: &str) {
    let (
        current_dir_path,
        component_dir_path, 
        component_rs_path, 
        path_to_html_from_root, 
        pascal_case_component_name, 
        kebab_case_component_name,
        snake_case_component_name
    ) = process_path(path, ObjectType::Service);

    create_object_module(&component_dir_path, &current_dir_path);
    create_service_rs_file(&component_rs_path, &pascal_case_component_name);
}

fn create_service_rs_file(
    service_rs_path: &PathBuf,
    pascal_case_component_name: &str,
) {
    let service_rs_contents = format!(r#"
pub struct {pascal_case_component_name} {{

}}

impl {pascal_case_component_name} {{
    pub fn new() -> Self {{
        Self {{

        }}
    }}

}}
    "#);

    fs::write(&service_rs_path, service_rs_contents)
     .expect("Failed to write service.rs file");
}