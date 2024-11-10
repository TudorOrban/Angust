
use std::collections::HashMap;

use angust::rendering::elements::component::component_factory_registry::initialize_component_registry;

use crate::app::app_component::AppComponent;
use crate::app::features::home::components::home_component::home_component::HomeComponent;
use crate::app::features::dashboard::components::dashboard_component::dashboard_component::DashboardComponent;
use crate::app::features::products::components::products_component::products_component::ProductsComponent;
use crate::app::features::settings::components::settings_component::settings_component::SettingsComponent;

/*
 * Function for registering all user-defined components. Should be called before Application::new()
 */
pub fn register_components() {
    let mut registry = HashMap::new();

    AppComponent::register(&mut registry);
    HomeComponent::register(&mut registry);
    DashboardComponent::register(&mut registry);
    ProductsComponent::register(&mut registry);
    SettingsComponent::register(&mut registry);

    initialize_component_registry(registry);
}
