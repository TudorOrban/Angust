
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct DashboardComponentState {
    content: String,
}

impl DashboardComponentState {

}

pub struct DashboardComponent {

}

impl DashboardComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("dashboard-component".to_string(), Box::new(move || {
            let state_factory = || DashboardComponentState::new(
                "dashboard-component works!".to_string(),
            );

            let component = Component::new(
                "dashboard-component".to_string(),
                "src/app/features/dashboard/components/dashboard_component/dashboard_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    