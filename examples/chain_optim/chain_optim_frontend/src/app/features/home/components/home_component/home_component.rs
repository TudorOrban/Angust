
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct HomeComponentState {
    content: String,
}

impl HomeComponentState {

}

pub struct HomeComponent {

}

impl HomeComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("home-component".to_string(), Box::new(move || {
            let state_factory = || HomeComponentState::new(
                "home-component works!".to_string(),
            );

            let component = Component::new(
                "home-component".to_string(),
                "src/app/features/home/components/home_component/home_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    