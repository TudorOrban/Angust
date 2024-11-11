
use std::collections::HashMap;

use angust::rendering::elements::{component::{
    component::Component, 
    component_factory_registry::ComponentFactory, 
}, service::{async_manager::FutureExt, service_registry::get_global_service}};
use angust_macros::component_state;

use crate::app::features::products::{models::product::Product, services::product_service::product_service::ProductService};


#[component_state]
struct ProductsComponentState {
    products: Vec<Product>,
}

impl ProductsComponentState {

    fn init(&mut self) {
        let product_service: &ProductService = get_global_service("product-service").unwrap();

        product_service.get_products()
            .post_to_gui_thread(|products: Vec<Product>| {
                self.products = products;
            });
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

            let component = Component::new(
                "products-component".to_string(),
                "src/app/features/products/components/products_component/products_component.html".to_string(),
                state_factory() 
            );

            Box::new(component)
        }));
    }
}
    