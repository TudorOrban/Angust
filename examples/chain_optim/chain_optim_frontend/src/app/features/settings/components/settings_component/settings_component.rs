
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct SettingsComponentState {
    content: String,
}

impl SettingsComponentState {

}

pub struct SettingsComponent {

}

impl SettingsComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("settings-component".to_string(), Box::new(move || {
            let state_factory = || SettingsComponentState::new(
                "settings-component works!".to_string(),
            );

            let component = Component::new(
                "settings-component".to_string(),
                "src/app/features/settings/components/settings_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    