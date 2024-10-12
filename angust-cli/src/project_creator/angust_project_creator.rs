use std::{env, fs, path::PathBuf, process::Command};

use super::types;


pub fn create_project(name: &str) {
    println!("Creating a new Angust project with name: {}", name);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_root_path = current_dir.join(name);

    let is_success = create_root_directory(&project_root_path, name);
    if !is_success {
        return;
    }

    initialize_cargo_project(&project_root_path);

    create_angust_configuration_file(&project_root_path);

}

fn create_root_directory(project_root_path: &PathBuf, name: &str) -> bool {
    if project_root_path.exists() {
        if project_root_path.is_dir() {
            eprintln!("A directory with the name '{}' already exists.", name);
            false
        } else {
            eprintln!("A file with the name '{}' already exists.", name);
            false
        }
    } else {
        match fs::create_dir_all(&project_root_path) {
            Ok(_) => {
                println!("Project directory created at: {}", project_root_path.display());
                true
            }
            Err(e) => {
                eprintln!("Failed to create project directory: {}", e);
                false
            }
        }
    }
}

fn initialize_cargo_project(project_root_path: &PathBuf) {
    match Command::new("cargo").arg("--version").output() {
        Ok(output) => println!("Found Cargo: {}", String::from_utf8_lossy(&output.stdout)),
        Err(_) => {
            eprintln!("Cargo is not installed or not found in PATH.");
            return;
        }
    }

    // Run `cargo init` in the specified directory
    let status = Command::new("cargo")
        .arg("init")
        .current_dir(project_root_path)
        .status()
        .expect("Failed to execute cargo init");

    if status.success() {
        println!("Cargo project initialized successfully.");
    } else {
        eprintln!("Failed to initialize cargo project.");
    }
}

fn create_angust_configuration_file(project_root_path: &PathBuf) {
    let config_file_path = project_root_path.join("angust.config.json");

    let default_config = types::AngustConfiguration::default();

    let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default configuration");

    std::fs::write(&config_file_path, config_json).expect("Failed to write configuration file");
}