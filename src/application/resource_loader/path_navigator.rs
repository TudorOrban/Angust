use std::env;

pub fn get_angust_config_path(relative_pathname: String) -> String {
    identify_project_root_path() + relative_pathname.as_str()
}

pub fn get_image_directory_path(relative_pathname: String) -> String {
    identify_project_root_path() + "/" + relative_pathname.as_str()
}

pub fn get_html_directory_path(relative_pathname: String) -> String {
    identify_project_root_path() + "/" + relative_pathname.as_str()
}

pub fn get_styles_path(relative_pathname: String) -> String {
    identify_project_root_path() + "/" + relative_pathname.as_str()
}

pub fn identify_project_root_path() -> String {
    env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        // Fallback: Use the directory where the executable is located
        env::current_exe()
            .expect("Failed to find executable path")
            .parent()
            .expect("Failed to resolve executable directory")
            .to_path_buf()
            .display()
            .to_string()
    })
}