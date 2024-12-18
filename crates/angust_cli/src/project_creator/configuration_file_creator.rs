use std::{fs, path::PathBuf, process::Command};

use toml_edit::{value, ArrayOfTables, DocumentMut, Item, Table};

use crate::shared::types::AngustConfiguration;

pub fn create_configuration_files(
    project_root_path: &PathBuf, 
    project_name: String, 
    angust_version: &str, 
    angust_macros_version: &str
) -> AngustConfiguration {
    create_git_ignore_file(&project_root_path);

    initialize_cargo_project(&project_root_path);

    adjust_cargo_toml(&project_root_path, &project_name, angust_version, angust_macros_version);

    create_angust_configuration_file(&project_root_path)
}

fn create_git_ignore_file(project_root_path: &PathBuf) {
    let git_ignore_path = project_root_path.join(".gitignore");

    let git_ignore_contents = r#"/target"#;

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

fn adjust_cargo_toml(
    project_root_path: &PathBuf, 
    project_name: &String, 
    angust_version: &str, 
    angust_macros_version: &str
) {
    let cargo_toml_path = project_root_path.join("Cargo.toml");

    let cargo_toml_contents = fs::read_to_string(&cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let mut doc = cargo_toml_contents.parse::<DocumentMut>()
        .expect("Failed to parse Cargo.toml");
 
    // Modify the package name and version
    doc["package"]["name"] = value(project_name);
    doc["package"]["version"] = value("0.1.0");
    doc["package"]["edition"] = value("2021");

    // Specify dependency to be a local path for development purposes
    doc["dependencies"]["angust"] = value(format!("{}", angust_version));
    doc["dependencies"]["angust_macros"] = value(format!("{}", angust_macros_version));
    doc["dependencies"]["tokio"] = value("1.41.0");

    // Define a binary target
    let bins = if let Some(array) = doc["bin"].as_array_of_tables_mut() {
        array
    } else {
        let aot = ArrayOfTables::new();
        doc["bin"] = Item::ArrayOfTables(aot.clone());
        doc["bin"].as_array_of_tables_mut().unwrap()
    };

    let mut bin_table = Table::new();
    bin_table["name"] = value(project_name).into();
    bin_table["path"] = value("src/main.rs").into();
    bins.push(bin_table);

    fs::write(&cargo_toml_path, doc.to_string())
        .expect("Failed to write modified Cargo.toml");
}

fn create_angust_configuration_file(project_root_path: &PathBuf) -> AngustConfiguration {
    let config_file_path = project_root_path.join("angust.config.json");

    let default_config = AngustConfiguration::default();

    let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default configuration");

    std::fs::write(&config_file_path, config_json).expect("Failed to write configuration file");

    default_config
}