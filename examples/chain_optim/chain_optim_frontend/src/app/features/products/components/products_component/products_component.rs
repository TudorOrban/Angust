
use std::collections::HashMap;

use angust::{rendering::elements::{
    component::{
        component::Component, 
        component_factory_registry::ComponentFactory, functions::component_functions::ComponentFunctions, 
    }, 
    service::service_registry::get_global_service
}, wrap_init_mut};
use angust_macros::component_state;

use crate::app::features::products::{models::product::Product, services::product_service::product_service::ProductService};


#[component_state]
struct ProductsComponentState {
    products: Vec<Product>,
}

impl ProductsComponentState {

    fn init(&mut self) {
        let product_service: &ProductService = get_global_service("product-service").unwrap();
        
        self.products = product_service.get_products();

        println!("Loaded products: {:?}", self.products);
    }
}

pub struct ProductsComponent {

}

impl ProductsComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("products-component".to_string(), Box::new(move || {
            let state_factory = || ProductsComponentState::new(
                vec![],
            );

            let mut component = Component::new(
                "products-component".to_string(),
                "src/app/features/products/components/products_component/products_component.html".to_string(),
                state_factory() 
            );

            let component_functions = ComponentFunctions::new(
                vec![], vec![], vec![], 
                vec![], vec![], 
                vec![],
                Some(wrap_init_mut!(ProductsComponentState, ProductsComponentState::init)),
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
    