
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct HeaderComponentState {
    content: String,
}

impl HeaderComponentState {

}

pub struct HeaderComponent {

}

impl HeaderComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("header-component".to_string(), Box::new(move || {
            let state_factory = || HeaderComponentState::new(
                "header-component works!".to_string(),
            );

            let component = Component::new(
                "header-component".to_string(),
                "src/app/core/header_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    