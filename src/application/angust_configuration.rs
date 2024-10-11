use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AngustConfiguration {
    pub angust_config_path: String,
    pub html_dir_relative_path: String,
    pub styles_relative_path: String,
    pub images_dir_relative_path: String,
}

impl Default for AngustConfiguration {
    fn default() -> Self {
        AngustConfiguration {
            angust_config_path: String::from("angust.configuration.json"),
            html_dir_relative_path: String::from("resources/html"),
            styles_relative_path: String::from("resources/styles/styles.css"),
            images_dir_relative_path: String::from("resources/images"),
        }
    }
}