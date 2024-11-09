
use std::{
    fs::{self, File}, 
    io::{Read, Write}, 
    path::PathBuf
};

use super::common::{create_object_module, process_path, ObjectType};




pub fn generate_service(path: &str) {
    let (
        current_dir_path,
        component_dir_path, 
        component_rs_path, 
        _,
        pascal_case_component_name, 
        _,
        _
    ) = process_path(path, ObjectType::Service);

    create_object_module(&component_dir_path, &current_dir_path);
    create_service_rs_file(&component_rs_path, &pascal_case_component_name);
    update_service_registration_module(&component_rs_path, &current_dir_path, &pascal_case_component_name);
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


fn update_service_registration_module(
    service_rs_path: &PathBuf, 
    current_dir_path: &PathBuf,
    pascal_case_service_name: &str
) {
    let relative_path = service_rs_path.strip_prefix(current_dir_path.join("src")).unwrap();
    let import_path = relative_path.to_str().unwrap()
                        .trim_end_matches(".rs")
                        .replace("\\", "/");  // Normalize path
    let module_path = import_path.replace("/", "::");

    let import_statement = format!("use crate::{}::{};", module_path, pascal_case_service_name);
    let register_call = format!("    registry.add_service(\"{}\", {}::new());", pascal_case_service_name, pascal_case_service_name);

    let service_registration_file_path = current_dir_path.join("src").join("service_registration.rs");

    let mut contents = String::new();
    if service_registration_file_path.exists() {
        File::open(&service_registration_file_path).unwrap().read_to_string(&mut contents).unwrap();
    }

    if !contents.contains(&import_statement) {
        let last_use_crate_index = contents.rfind("use crate").map(|idx| contents[idx..].find('\n').unwrap() + idx + 1).unwrap_or(0);
        contents.insert_str(last_use_crate_index, &format!("{}\n", import_statement));
    }
    
    let last_line_pattern = "initialize_service_registry(registry);";
    let init_call_index = contents.find(last_line_pattern).unwrap();
    if !contents[..init_call_index].contains(&register_call) {
        let last_register_index = contents[..init_call_index].rfind(';').unwrap() + 1;
        contents.insert_str(last_register_index, &format!("\n{}", register_call));
    }

    let init_line_start = contents.rfind(last_line_pattern).unwrap();
    contents.replace_range(init_line_start..init_line_start, "");

    File::create(&service_registration_file_path).unwrap().write_all(contents.as_bytes()).unwrap();
}
