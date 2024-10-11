use crate::application::app_configuration::AngustConfiguration;

use super::path_navigator::identify_project_root_path;


pub fn load_angust_configuration() -> AngustConfiguration {
    let project_root_path = identify_project_root_path();
    let angust_default_config = AngustConfiguration::default();
    let config_relative_path = angust_default_config.angust_config_path;

    let config_path = project_root_path + "/" + config_relative_path.as_str();

    match std::fs::read_to_string(&config_path) {
        Ok(config_content) => {
            serde_json::from_str(&config_content).unwrap_or_else( angust_default_config)
        },
        Err(_) => angust_default_config,
    }
}