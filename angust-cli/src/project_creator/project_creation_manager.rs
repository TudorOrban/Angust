use std::{env, fs, path::PathBuf};


use crate::project_creator::{app_directory_creator, assets_directory_creator, configuration_file_creator, core_file_creator, styles_directory_creator, utils::string_pascal_to_snake_case};



pub fn create_project(name: &str) {
    println!("Creating a new Angust project with name: {}", name);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let snake_case_name = string_pascal_to_snake_case(name);
    let project_root_path = current_dir.join(snake_case_name.clone());

    create_root_directory(&project_root_path, &snake_case_name);

    let angust_config = configuration_file_creator::create_configuration_files(&project_root_path, snake_case_name);
    
    core_file_creator::create_core_files(&project_root_path, &angust_config.pathing_config.main_rs_path, &angust_config.pathing_config.index_html_path);

    app_directory_creator::create_app_directory(&project_root_path, &angust_config.pathing_config.app_dir_path);
    
    assets_directory_creator::create_assets_directory(&project_root_path, &angust_config.pathing_config.assets_dir_path);
    
    styles_directory_creator::create_styles_directory(&project_root_path, &angust_config.pathing_config.styles_dir_path);   
}

fn create_root_directory(project_root_path: &PathBuf, name: &String) {
    if project_root_path.exists() {
        if project_root_path.is_dir() {
            panic!("A directory with the name '{}' already exists.", name);
        } else {
            panic!("A file with the name '{}' already exists.", name);
        }
    } else {
        match fs::create_dir_all(&project_root_path) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to create project directory: {}", e);
            }
        }
    }
}
