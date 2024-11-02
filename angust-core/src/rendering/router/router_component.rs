use std::{collections::HashMap, sync::Arc};

use crate::rendering::elements::{
    component::component_factory_registry::create_component, 
    element::Element
};

use super::angust_router::subscribe_to_current_route;


pub struct RouterComponent {
    current_component: Option<Box<dyn Element>>,
}

impl RouterComponent {
    pub fn new() -> Self {
        let component = RouterComponent {
            current_component: None,
        };

        let callback = Arc::new(move |route: &str, component_name: &str| {
            println!("Route changed to: {}, loading component: {}", route, component_name);
            
            let component_optional = create_component(component_name);
            if component_optional.is_none() {
                println!("Component not found: {}", component_name);
            }
            let mut component_box = component_optional.unwrap();
            
            println!("Component found: {}", component_name);
            component_box.initialize(HashMap::new());

            // component.current_component = Some(component_box);
        });
        subscribe_to_current_route(callback);

        component
    }
}