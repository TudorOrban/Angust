
use std::collections::HashMap;

use angust::{rendering::elements::component::{
    component::Component, 
    component_factory_registry::ComponentFactory, functions::component_functions::ComponentFunctions, 
}, wrap_fn_mut};
use angust_macros::component_state;

use crate::app::features::products::models::product::Product;


#[component_state]
struct ProductComponentState {
    product: Option<Product>
}

impl ProductComponentState {

    fn set_product(&mut self, product: Product) {
        self.product = Some(product);
    }

    fn edit_product(&mut self, product_id: u32) {
        println!("Editing product: {}", product_id);
    }
}

pub struct ProductComponent {

}

impl ProductComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("product-component".to_string(), Box::new(move || {
            let state = ProductComponentState::new(
                None
            );

            let mut component = Component::new(
                "product-component".to_string(),
                "src/app/features/products/components/product_component/product_component.html".to_string(),
                state
            );

            let component_functions = ComponentFunctions::new(
                vec![], vec![], vec![], 
                vec![], 
                vec![
                    ("edit_product", wrap_fn_mut!(ProductComponentState, ProductComponentState::edit_product, u32))
                ], 
                vec![
                    ("set_product", wrap_fn_mut!(ProductComponentState, ProductComponentState::set_product, Product))
                ],
                None
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
    