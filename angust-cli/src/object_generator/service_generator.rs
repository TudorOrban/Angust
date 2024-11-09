use std::{env, path::PathBuf};




pub fn generate_service(path: &str) {
    println!("Service {} would be generated here.", path);

    
}

fn process_path(path: &str) -> String {
    let provided_path = PathBuf::from(path);
    let current_dir_path = env::current_dir().expect("Failed to get current directory");

    "".to_string()
}