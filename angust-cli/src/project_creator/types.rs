use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AngustConfiguration {
    pub angust_config_path: String,
    pub app_dir_path: String,
    pub assets_dir_path: String,
    pub styles_dir_path: String,
    pub main_rs_path: String,
    pub index_html_path: String,
}

impl Default for AngustConfiguration {
    fn default() -> Self {
        AngustConfiguration {
            angust_config_path: String::from("angust.configuration.json"),
            app_dir_path: String::from("src/app"),
            assets_dir_path: String::from("src/assets"),
            styles_dir_path: String::from("src/styles"),
            main_rs_path: String::from("src/main.rs"),
            index_html_path: String::from("src/index.html"),
        }
    }
}