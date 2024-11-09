use std::{
    fs::{self, File}, 
    io::{Read, Write}, 
    path::{Path, PathBuf}
};

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