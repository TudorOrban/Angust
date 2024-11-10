
use std::collections::HashMap;

use angust::rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
};
use angust_macros::component_state;


#[component_state]
struct ProductsComponentState {
    content: String,
}

impl ProductsComponentState {

}

pub struct ProductsComponent {

}

impl ProductsComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("products-component".to_string(), Box::new(move || {
            let state_factory = || ProductsComponentState::new(
                "products-component works!".to_string(),
            );

            let component = Component::new(
                "products-component".to_string(),
                "src/app/features/products/components/products_component/products_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    