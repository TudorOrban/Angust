
use std::collections::HashMap;

use angust::rendering::elements::component::component_factory_registry::initialize_component_registry;

use crate::app::app_component::AppComponent;

/*
 * Function for registering all user-defined components. Should be called before Application::new()
 */
pub fn register_components() {
    let mut registry = HashMap::new();

    AppComponent::register(&mut registry);

    initialize_component_registry(registry);
}
