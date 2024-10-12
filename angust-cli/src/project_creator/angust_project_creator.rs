use std::{env, fs, path::PathBuf, process::Command};

use toml_edit::{value, DocumentMut};

use crate::project_creator::utils::string_pascal_to_snake_case;

use super::types::{self, AngustConfiguration};


pub fn create_project(name: &str) {
    println!("Creating a new Angust project with name: {}", name);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let snake_case_name = string_pascal_to_snake_case(name);
    let project_root_path = current_dir.join(snake_case_name.clone());

    let is_success = create_root_directory(&project_root_path, &snake_case_name);
    if !is_success {
        return;
    }

    create_git_ignore_file(&project_root_path);
    initialize_cargo_project(&project_root_path);
    adjust_cargo_toml(&project_root_path, &snake_case_name);
    let angust_config = create_angust_configuration_file(&project_root_path);
    create_src_directory(&project_root_path, &angust_config);
}

fn create_root_directory(project_root_path: &PathBuf, name: &String) -> bool {
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

fn create_git_ignore_file(project_root_path: &PathBuf) {
    let git_ignore_path = project_root_path.join(".gitignore");

    let git_ignore_contents = r#"target/"#;

    fs::write(&git_ignore_path, git_ignore_contents)
        .expect("Failed to write .gitignore file");
}

fn initialize_cargo_project(project_root_path: &PathBuf) {
    match Command::new("cargo").arg("--version").output() {
        Ok(_) => {}
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

fn adjust_cargo_toml(project_root_path: &PathBuf, name: &String) {
    let cargo_toml_path = project_root_path.join("Cargo.toml");

    let cargo_toml_contents = fs::read_to_string(&cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let mut doc = cargo_toml_contents.parse::<DocumentMut>()
        .expect("Failed to parse Cargo.toml");

    // Modify the package name and version
    doc["package"]["name"] = value(name);
    doc["package"]["version"] = value("0.1.0");
    doc["package"]["edition"] = value("2021");

    // Specify dependency to be a local path for development purposes
    doc["dependencies"]["angust"] = value("../../../angust-core/");

    // Define a binary target
    let bin_table = doc["bin"].or_insert(toml_edit::table());
    bin_table["name"] = value(name);
    bin_table["path"] = value("src/main.rs");

    fs::write(&cargo_toml_path, doc.to_string())
        .expect("Failed to write modified Cargo.toml");
}

fn create_angust_configuration_file(project_root_path: &PathBuf) -> AngustConfiguration {
    let config_file_path = project_root_path.join("angust.config.json");

    let default_config = types::AngustConfiguration::default();

    let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default configuration");

    std::fs::write(&config_file_path, config_json).expect("Failed to write configuration file");

    default_config
}

fn create_src_directory(project_root_path: &PathBuf, angust_config: &AngustConfiguration) {
    create_app_directory(project_root_path, &angust_config.app_dir_path);
    create_assets_directory(project_root_path, &angust_config.assets_dir_path);
    create_styles_directory(project_root_path, &angust_config.styles_dir_path);
    create_index_html_file(project_root_path, &angust_config.index_html_path);
}

fn create_app_directory(project_root_path: &PathBuf, app_folder_path: &String) {
    let app_dir_path = project_root_path.join(app_folder_path);

    match fs::create_dir_all(&app_dir_path) {
        Ok(_) => {
            println!("App directory created at: {}", app_dir_path.display());
        }
        Err(e) => {
            eprintln!("Failed to create app directory: {}", e);
        }
    }
}

fn create_assets_directory(project_root_path: &PathBuf, assets_folder_path: &String) {
    let assets_dir_path = project_root_path.join(assets_folder_path);

    match fs::create_dir_all(&assets_dir_path) {
        Ok(_) => {
            println!("Assets directory created at: {}", assets_dir_path.display());
        }
        Err(e) => {
            eprintln!("Failed to create assets directory: {}", e);
        }
    }
}

fn create_styles_directory(project_root_path: &PathBuf, styles_folder_path: &String) {
    let styles_dir_path = project_root_path.join(styles_folder_path);

    match fs::create_dir_all(&styles_dir_path) {
        Ok(_) => {
            println!("Styles directory created at: {}", styles_dir_path.display());
        }
        Err(e) => {
            eprintln!("Failed to create styles directory: {}", e);
        }
    }
}

fn create_index_html_file(project_root_path: &PathBuf, index_html_path: &String) {
    let index_html_path = project_root_path.join(index_html_path);

    let index_html_contents = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Angust App</title>
</head>
<body>
    <div>
        <h1>Welcome to Angust!</h1>
    </div>
</body>
</html>
"#;

    fs::write(&index_html_path, index_html_contents)
        .expect("Failed to write index.html file");
}