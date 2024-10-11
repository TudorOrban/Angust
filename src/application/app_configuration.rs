use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AngustConfiguration {
    pub angust_config_path: String,
    pub html_dir_relative_path: String,
    pub stylesheet_relative_path: String,
    pub image_relative_path: String,
}

impl Default for AngustConfiguration {
    fn default() -> Self {
        AngustConfiguration {
            angust_config_path: String::from("angust.configuration.json"),
            html_dir_relative_path: String::from("resources/html"),
            stylesheet_relative_path: String::from("resources/styles/styles.css"),
            image_relative_path: String::from("resources/images"),
        }
    }
}