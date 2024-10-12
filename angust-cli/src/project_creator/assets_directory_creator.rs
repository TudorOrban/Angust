use std::{fs, path::PathBuf};


pub fn create_assets_directory(project_root_path: &PathBuf, assets_folder_path: &String) {
    let assets_dir_path = project_root_path.join(assets_folder_path);

    match fs::create_dir_all(&assets_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create assets directory: {}", e);
        }
    }

    let assets_images_dir_path = assets_dir_path.join("img");

    match fs::create_dir_all(&assets_images_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create assets/images directory: {}", e);
        }
    }

    let assets_fonts_dir_path = assets_dir_path.join("fonts");

    match fs::create_dir_all(&assets_fonts_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create assets/fonts directory: {}", e);
        }
    }
}
